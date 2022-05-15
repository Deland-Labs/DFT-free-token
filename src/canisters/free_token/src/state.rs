use candid::{CandidType, Deserialize, Nat, Principal};
use getset::{Getters, Setters};
use num_bigint::BigUint;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use crate::permissions::MintError;
use crate::received_reward_store::{ReceivedRewardRecordStore, ReceivesRewardRecord, ReceivesRewardRecordState};
use crate::reward_store::{RewardCode, RewardPackage, RewardStore};
use crate::service::CommonResult;
use crate::TimeInNs;
thread_local! {
    pub static STATE: State = State::default();
}


pub struct TransactionId(pub String);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct User(pub Principal);


impl From<Principal> for User {
    fn from(principal: Principal) -> Self {
        User(principal)
    }
}

impl From<User> for Principal {
    fn from(user: User) -> Self {
        user.0
    }
}


#[derive(Default)]
pub struct State {
    pub(crate) received_reward_record_store: RefCell<ReceivedRewardRecordStore>,
    pub(crate) unlimited_user_store: RefCell<UnlimitedUserStore>,
    pub(crate) reward_store: RefCell<RewardStore>,
}


impl State {
    pub fn is_able_receive(&self, user: &User, reward_code: &RewardCode) -> CommonResult<()> {
        let state = self;
        let reward_store = state.reward_store.borrow();
        if !reward_store.is_reward_available(reward_code) {
            return Err(MintError::RewardCodeNotAvailable).into();
        }
        let unlimited_user_store = state.unlimited_user_store.borrow();
        let received_reward_record_store = state.received_reward_record_store.borrow();
        if received_reward_record_store.is_received_state_all_completed(user, reward_code) {
            return Err(MintError::RewardIncomplete).into();
        }

        if received_reward_record_store.is_received_reward_record_exist(user, reward_code) == true
            || unlimited_user_store.is_unlimited_user(user) == false {
            return Err(MintError::RewardAlreadyReceived).into();
        }
        Ok(())
    }
    pub fn receive_reward(&self, user: &User, reward_code: &RewardCode, time: TimeInNs) -> CommonResult<ReceivesRewardRecord> {
        let state = self;
        let mut received_reward_record_store = state.received_reward_record_store.borrow_mut();
        let reward_store = state.reward_store.borrow();
        let reward_package = reward_store.get_reward(reward_code);
        if reward_package.is_none() {
            return Err(MintError::RewardCodeNotAvailable).into();
        }

        let mut reward_record_hash = HashMap::new();
        for reward_type in reward_package.unwrap().reward_types().iter() {
            reward_record_hash.insert(reward_type.clone(), ReceivesRewardRecordState::Sending);
        }

        let reward_record = ReceivesRewardRecord::new(reward_record_hash, time);
        received_reward_record_store.add_received_reward_record(user.clone(), reward_code.clone(), reward_record.clone());
        Ok(reward_record)
    }
}


#[derive(Default)]
pub struct UnlimitedUserStore {
    unlimited_users: HashMap<User, Vec<RewardCode>>,
}

impl UnlimitedUserStore {
    pub fn add_unlimited_user(&mut self, user: User, reward_codes: Vec<RewardCode>) {
        self.unlimited_users
            .entry(user)
            .or_insert(Vec::new())
            .extend(reward_codes);
    }
    pub fn remove_unlimited_user(&mut self, user: User) {
        self.unlimited_users.remove(&user);
    }
    pub fn is_unlimited_user(&self, user: &User) -> bool {
        self.unlimited_users.contains_key(user)
    }
}


pub struct UnlimitedUser {
    user: User,
    reward_codes: Vec<RewardCode>,
}