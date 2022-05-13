use std::collections::HashMap;
use candid::Principal;
use crate::reward_store::{RewardCode, RewardStore};
use crate::state::User;

#[derive(Default)]
pub struct ReceivedRewardRecordStore {
    received_reward_records: HashMap<RewardCode, HashMap<User, Vec<ReceivesRewardRecord>>>,
}

impl ReceivedRewardRecordStore {
    pub fn new() -> Self {
        Self {
            received_reward_records: HashMap::new(),
        }
    }
    pub fn add_received_reward_record(&mut self, user: &User, reward_store: &RewardCode, received_reward_record: &ReceivesRewardRecord) {
        self.received_reward_records
            .entry(reward_store.into())
            .or_insert_with(HashMap::new())
            .entry(user.into())
            .or_insert_with(Vec::new)
            .push(received_reward_record.clone());
    }

    pub fn get_received_reward_records(&self, reward_store: &RewardCode, user: &User) -> Option<&Vec<ReceivesRewardRecord>> {
        self.received_reward_records.get(reward_store).and_then(|received_reward_records| received_reward_records.get(user))
    }
    pub fn is_received_reward_record_exist(&self, user: &User, reward_store: &RewardCode) -> bool {
        self.get_received_reward_records(reward_store, user)
            .map(|received_reward_records| received_reward_records.iter().any())
            .unwrap_or(false)
    }
    pub fn is_received_state_sending_exist(&self, user: &User, reward_store: &RewardCode) -> bool {
        self.get_received_reward_records(reward_store, user)
            .map(|received_reward_records| {
                received_reward_records.iter().any(|received_reward_record| {
                    received_reward_record.is_sending()
                })
            })
            .unwrap_or(false)
    }
}

pub enum ReceivesRewardRecordState {
    Sending,
    Completed,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ReceivesRewardRecord {
    state: ReceivesRewardRecordState,
    created_at: Option<u64>,
}


impl ReceivesRewardRecord {
    pub fn new(created_at: Option<u64>, state: ReceivesRewardRecordState) -> Self {
        Self {
            created_at,
            state,
        }
    }
    pub fn is_sending(&self) -> bool {
        self.state == ReceivesRewardRecordState::Sending
    }
}