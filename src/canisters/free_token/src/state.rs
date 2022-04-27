use candid::{CandidType, Deserialize, Nat, Principal};
use getset::{Getters, Setters};
use num_bigint::BigUint;
use std::cell::RefCell;
use std::collections::HashMap;
thread_local! {
    pub static STATE: State = State::default();
}

pub type TransactionId = String;
type User = Principal;

#[derive(Default)]
pub struct State {
    pub free_records: RefCell<HashMap<User, FreeTokenRecord>>,
    pub free_settings: RefCell<FreeSetting>,
    pub unlimited_users: RefCell<Vec<User>>,
}

impl State {}

#[derive(Eq, PartialEq, Debug, Clone, Setters, Getters)]
pub struct FreeTokenRecord {
    #[getset(get = "pub")]
    from: Principal,
    #[getset(get = "pub")]
    amount: Nat,
    #[getset(get = "pub")]
    created_at: Option<u64>,
    #[getset(get = "pub")]
    user: Principal,
}

impl FreeTokenRecord {
    pub fn new(from: Principal, amount: Nat, created_at: Option<u64>, user: Principal) -> Self {
        FreeTokenRecord {
            from,
            amount,
            created_at,
            user,
        }
    }
}

#[derive(Debug, Clone, Setters, Getters)]
pub struct FreeSetting {
    #[getset(get = "pub")]
    pub free_amount: Nat,
    #[getset(get = "pub")]
    pub minter: Principal,
}

impl FreeSetting {
    pub fn new(free_amount: Nat, minter: Principal) -> Self {
        FreeSetting {
            free_amount,
            minter,
        }
    }
}

impl Default for FreeSetting {
    fn default() -> Self {
        FreeSetting {
            free_amount: Nat::from(0),
            minter: Principal::anonymous(),
        }
    }
}
