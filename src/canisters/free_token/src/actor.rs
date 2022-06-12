use crate::canister_api::OperationResult;
use crate::ic_logger::ICLogger;
use crate::permissions::{ErrorInfo, FreeTokenError};
use crate::received_reward_store::ReceivesRewardRecord;
use crate::reward_store::{RewardCode, RewardPackage};
use crate::service::{CommonResult, FreeTokenService};
use crate::state::State;
use crate::TimeInNs;
use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::*;
use log::{debug, logger, LevelFilter};
use std::collections::HashMap;
use std::panic;
use yansi::Paint;

#[init]
#[candid_method(init)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::clone_on_copy)]
fn canister_init() {
    log::set_logger(&ICLogger);
    log::set_max_level(LevelFilter::Trace);
    panic::set_hook(Box::new(|data| {
        let message = format!("{}", data);
        api::print(Paint::red(message).to_string());
    }));
}

#[update(name = "receive_free_token")]
#[candid_method(update)]
async fn receive_free_token(reward_code: RewardCode) -> BooleanResult {
    let caller = api::caller();
    let now = api::time();
    let service = FreeTokenService::default();

    let result = service
        .receive_free_token(&caller, &reward_code, TimeInNs(now))
        .await;
    result.into()
}

#[update(name = "add_reward")]
#[candid_method(update)]
async fn add_reward(
    reward_code: RewardCode,
    reward_package: RewardPackage,
    unlimited_users: Option<Vec<Principal>>,
) -> BooleanResult {
    let caller = api::caller();
    let service = FreeTokenService::default();
    let result = service
        .add_reward(&caller, reward_code, reward_package, unlimited_users)
        .await;
    result.into()
}

#[query(name = "get_reward_packages")]
#[candid_method(query)]
fn get_reward_packages() -> RewardPackagesResult {
    let caller = api::caller();
    let service = FreeTokenService::default();
    let result = service.get_reward_packages(&caller);
    result.into()
}

#[query(name = "get_reward_package")]
#[candid_method(query)]
fn get_reward_package(reward_code: RewardCode) -> RewardPackageResult {
    let service = FreeTokenService::default();
    let result = service.get_reward_package(&reward_code);
    result.into()
}

#[query(name = "history")]
#[candid_method(query)]
fn history() -> HistoryResult {
    let caller = api::caller();
    let service = FreeTokenService::default();
    let result = service.history(&caller);
    result.into()
}
#[derive(CandidType, Debug, Deserialize)]
pub enum BooleanResult {
    Ok(bool),
    Err(ErrorInfo),
}

impl From<CommonResult<bool>> for BooleanResult {
    fn from(result: CommonResult<bool>) -> Self {
        match result {
            Ok(value) => BooleanResult::Ok(value),
            Err(error) => BooleanResult::Err(error.into()),
        }
    }
}
#[derive(CandidType, Debug, Deserialize)]
pub enum RewardPackagesResult {
    Ok(HashMap<RewardCode, RewardPackage>),
    Err(ErrorInfo),
}

impl From<CommonResult<HashMap<RewardCode, RewardPackage>>> for RewardPackagesResult {
    fn from(result: CommonResult<HashMap<RewardCode, RewardPackage>>) -> Self {
        match result {
            Ok(value) => RewardPackagesResult::Ok(value),
            Err(error) => RewardPackagesResult::Err(error.into()),
        }
    }
}
#[derive(CandidType, Debug, Deserialize)]
pub enum RewardPackageResult {
    Ok(RewardPackage),
    Err(ErrorInfo),
}

impl From<CommonResult<RewardPackage>> for RewardPackageResult {
    fn from(result: CommonResult<RewardPackage>) -> Self {
        match result {
            Ok(value) => RewardPackageResult::Ok(value),
            Err(error) => RewardPackageResult::Err(error.into()),
        }
    }
}

#[derive(CandidType, Debug, Deserialize)]
pub enum HistoryResult {
    Ok(HashMap<RewardCode, ReceivesRewardRecord>),
    Err(ErrorInfo),
}

impl From<CommonResult<HashMap<RewardCode, ReceivesRewardRecord>>> for HistoryResult {
    fn from(result: CommonResult<HashMap<RewardCode, ReceivesRewardRecord>>) -> Self {
        match result {
            Ok(value) => HistoryResult::Ok(value),
            Err(error) => HistoryResult::Err(error.into()),
        }
    }
}

candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query)]
fn __export_did_tmp_() -> String {
    __export_service()
}
