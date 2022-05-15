use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use candid::{Nat, Principal, CandidType, Deserialize, Error};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RewardCode(pub String);

pub struct RewardPackage(Vec<RewardType>);

impl RewardPackage {
    pub fn new(reward_types: Vec<RewardType>) -> Self {
        RewardPackage(reward_types)
    }
    pub fn reward_types(&self) -> &Vec<RewardType> {
        &self.0
    }
}

#[derive(Default)]
pub struct RewardStore {
    rewards: HashMap<RewardCode, RewardPackage>,
}

impl RewardStore {
    pub fn new() -> Self {
        RewardStore {
            rewards: HashMap::new(),
        }
    }

    pub fn add_reward(&mut self, code: RewardCode, package: RewardPackage) {
        self.rewards.insert(code, package);
    }

    pub fn get_reward(&self, code: &RewardCode) -> Option<&RewardPackage> {
        self.rewards.get(code)
    }

    pub fn get_mut_reward(&mut self, code: &RewardCode) -> Option<&mut RewardPackage> {
        self.rewards.get_mut(code)
    }

    pub fn is_reward_available(&self, code: &RewardCode) -> bool {
        self.rewards.contains_key(code)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, Hash, PartialEq, CandidType, Deserialize)]
pub enum RewardType {
    QuotaRewardPackage {
        canister: Principal,
        quota_type: QuotaType,
        diff: u32,
    },
    TokenMintRewardPackage {
        canister: Principal,
        amount: Nat,
    },
    TokenTransferRewardPackage {
        canister: Principal,
        amount: Nat,
    },
}


/// Quota type to be used for registration
#[derive(Deserialize, Copy, CandidType, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum QuotaType {
    /// The length of name's the first part in chars must be equal to the value.
    /// e.g. LenEq(3) means that the first part of the name must be 3 chars long.
    LenEq(u8),
    /// The length of name's the first part in chars must be more than or equal to the value.
    /// e.g. LenGt(3) means that the first part of the name must be at least 3 chars long.
    LenGte(u8),
}

impl Display for QuotaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotaType::LenEq(len) => write!(f, "len_eq({})", len),
            QuotaType::LenGte(len) => write!(f, "len_gte({})", len),
        }
    }
}

impl QuotaType {
    pub fn len(&self) -> u8 {
        match self {
            QuotaType::LenEq(len) => *len,
            QuotaType::LenGte(len) => *len,
        }
    }
}