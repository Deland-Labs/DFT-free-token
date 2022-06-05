use crate::permissions::FreeTokenError;
use crate::received_reward_store::{
    ReceivedRewardRecordStore, ReceivesRewardRecord, ReceivesRewardRecordState,
};
use crate::reward_store::{RewardCode, RewardPackage, RewardStore};
use crate::service::CommonResult;
use crate::TimeInNs;
use candid::{CandidType, Deserialize, Nat, Principal};
use getset::{Getters, Setters};
use num_bigint::BigUint;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
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
            return Err(FreeTokenError::RewardCodeNotAvailable).into();
        }
        let unlimited_user_store = state.unlimited_user_store.borrow();
        let received_reward_record_store = state.received_reward_record_store.borrow();
        if unlimited_user_store.is_unlimited_user(reward_code.clone(), user) {
            return Ok(());
        }

        if !received_reward_record_store.is_received_state_all_completed(user, reward_code) {
            return Err(FreeTokenError::RewardIncomplete).into();
        }

        let is_exist =
            received_reward_record_store.is_received_reward_record_exist(user, reward_code);
        if is_exist {
            return Err(FreeTokenError::RewardAlreadyReceived).into();
        }
        Ok(())
    }
    pub fn receive_reward(
        &self,
        user: &User,
        reward_code: &RewardCode,
        time: TimeInNs,
    ) -> CommonResult<ReceivesRewardRecord> {
        let state = self;
        let mut received_reward_record_store = state.received_reward_record_store.borrow_mut();
        let reward_store = state.reward_store.borrow();
        let reward_package = reward_store.get_reward_package(reward_code);
        if reward_package.is_none() {
            return Err(FreeTokenError::RewardCodeNotAvailable).into();
        }

        let mut reward_record_hash = HashMap::new();
        for reward_type in reward_package.unwrap().reward_types().iter() {
            reward_record_hash.insert(reward_type.clone(), ReceivesRewardRecordState::Sending);
        }

        let reward_record = ReceivesRewardRecord::new(reward_record_hash, time);
        received_reward_record_store.add_received_reward_record(
            user.clone(),
            reward_code.clone(),
            reward_record.clone(),
        );
        println!("receive_reward: {:?}", reward_record);
        Ok(reward_record)
    }
}

#[derive(Default)]
pub struct UnlimitedUserStore {
    unlimited_users: HashMap<RewardCode, Vec<User>>,
}

impl UnlimitedUserStore {
    pub fn add_unlimited_user(&mut self, reward_code: RewardCode, users: Vec<User>) {
        self.unlimited_users.insert(reward_code, users);
    }
    pub fn remove_unlimited_user(&mut self, reward_code: RewardCode, user: &User) {
        if let Some(users) = self.unlimited_users.get_mut(&reward_code) {
            users.retain(|u| u != user);
        }
    }
    pub fn is_unlimited_user(&self, reward_code: RewardCode, user: &User) -> bool {
        return if let Some(users) = self.unlimited_users.get(&reward_code) {
            users.contains(user)
        } else {
            false
        };
    }
}
