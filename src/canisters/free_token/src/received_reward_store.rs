use std::collections::HashMap;
use std::fmt::Debug;
use candid::{Principal, CandidType, Deserialize};
use crate::reward_store::{RewardCode, RewardStore, RewardType};
use crate::state::User;
use crate::TimeInNs;

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
    pub fn add_received_reward_record(&mut self, user: User, reward_store: RewardCode, received_reward_record: ReceivesRewardRecord) {
        self.received_reward_records
            .entry(reward_store)
            .or_insert_with(|| HashMap::new())
            .entry(user)
            .or_insert_with(Vec::new)
            .push(received_reward_record);
    }

    pub fn get_received_reward_records(&self, reward_store: &RewardCode, user: &User) -> Option<&Vec<ReceivesRewardRecord>> {
        self.received_reward_records.get(reward_store).and_then(|received_reward_records| received_reward_records.get(user))
    }
    pub fn is_received_reward_record_exist(&self, user: &User, reward_store: &RewardCode) -> bool {
        self.get_received_reward_records(reward_store, user)
            .map(|received_reward_records| received_reward_records.iter().len() > 0)
            .unwrap_or(false)
    }
    pub fn is_received_state_all_completed(&self, user: &User, reward_store: &RewardCode) -> bool {
        self.get_received_reward_records(reward_store, user)
            .map(|received_reward_records| {
                received_reward_records.iter().all(|received_reward_record| {
                    received_reward_record.is_state_all_completed()
                })
            })
            .unwrap_or(false)
    }

    pub fn update_received_reward_record(&mut self, user: User, reward_code: RewardCode, received_reward_record: &ReceivesRewardRecord) {
        self.received_reward_records
            .entry(reward_code.clone())
            .or_insert_with(|| HashMap::new())
            .entry(user.clone())
            .or_insert_with(Vec::new)
            .retain(|received_reward_record| {
                received_reward_record.get_reward_type() != received_reward_record.get_reward_type()
            });
        self.received_reward_records
            .entry(reward_code)
            .or_insert_with(|| HashMap::new())
            .entry(user)
            .or_insert_with(Vec::new)
            .push(received_reward_record.clone());
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub enum ReceivesRewardRecordState {
    Sending,
    Completed,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ReceivesRewardRecord {
    rewards: HashMap<RewardType, ReceivesRewardRecordState>,
    created_at: TimeInNs,
}


impl ReceivesRewardRecord {
    pub fn new(rewards: HashMap<RewardType, ReceivesRewardRecordState>, created_at: TimeInNs) -> Self {
        Self {
            rewards,
            created_at,
        }
    }
    pub fn get_reward_type(&self) -> Vec<RewardType> {
        self.rewards.keys().cloned().collect()
    }
    pub fn set_reward_state_completed(&mut self, reward_type: &RewardType) {
        self.rewards.insert(reward_type.clone(), ReceivesRewardRecordState::Completed);
    }

    pub fn is_state_all_completed(&self) -> bool {
        self.rewards.values().all(|reward_state| {
            match reward_state {
                ReceivesRewardRecordState::Completed => true,
                _ => false,
            }
        })
    }
}