use candid::Nat;
use candid::Principal;
use const_env::from_env;
use log::info;
use num_bigint::BigUint;
use std::cell::RefCell;
use std::str::FromStr;

pub const PAGE_INPUT_MIN_LIMIT: usize = 1;
pub const PAGE_INPUT_MAX_LIMIT: usize = 100;
pub const PAGE_INPUT_MIN_OFFSET: usize = 0;
pub const PAGE_INPUT_MAX_OFFSET: usize = 10_000;

pub const NAMING_ENV_DEV: &str = "dev";
pub const NAMING_ENV_STAGING: &str = "staging";
pub const NAMING_ENV_PRODUCTION: &str = "production";

#[from_env]
pub const NAMING_ENV: &str = NAMING_ENV_DEV;

pub enum FreeTokenEnv {
    Dev,
    Staging,
    Production,
}

pub fn is_env(env: FreeTokenEnv) -> bool {
    match env {
        FreeTokenEnv::Dev => NAMING_ENV == NAMING_ENV_DEV,
        FreeTokenEnv::Staging => NAMING_ENV == NAMING_ENV_STAGING,
        FreeTokenEnv::Production => NAMING_ENV == NAMING_ENV_PRODUCTION,
    }
}

pub fn is_dev_env() -> bool {
    is_env(FreeTokenEnv::Dev)
}

#[from_env]
pub const FREE_TOKEN_PRINCIPAL_NAME_ADMIN: &str = "";
#[from_env]
pub const FREE_TOKEN_PRINCIPAL_NAME_STATE_EXPORTER: &str = "";
#[from_env]
pub const FREE_TOKEN_PRINCIPAL_NAME_TIMER_TRIGGER: &str = "";
