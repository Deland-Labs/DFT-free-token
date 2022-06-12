use candid::{candid_method, CandidType, Deserialize, Principal};
use std::fmt::Display;
use std::ops::{Add, Sub};

mod actor;
mod canister_api;
mod constants;
mod ic_logger;
mod permissions;
mod received_reward_store;
mod reward_store;
mod service;
mod state;

#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize, Copy, Ord, PartialOrd)]
#[serde(transparent)]
pub struct TimeInNs(pub u64);

impl Add for TimeInNs {
    type Output = TimeInNs;

    fn add(self, rhs: Self) -> Self::Output {
        TimeInNs(self.0 + rhs.0)
    }
}

impl Sub for TimeInNs {
    type Output = TimeInNs;

    fn sub(self, rhs: Self) -> Self::Output {
        TimeInNs(self.0 - rhs.0)
    }
}

impl Display for TimeInNs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ns", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize, Copy, Ord, PartialOrd)]
#[serde(transparent)]
pub struct TimeInSec(pub u64);

impl From<TimeInNs> for TimeInSec {
    fn from(ns: TimeInNs) -> Self {
        TimeInSec(ns.0 / 1_000_000_000)
    }
}

impl Display for TimeInSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} s", self.0)
    }
}
