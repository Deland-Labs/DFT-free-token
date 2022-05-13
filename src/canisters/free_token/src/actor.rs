use crate::canister_api::OperationResult;
use crate::ic_logger::ICLogger;
use crate::permissions::{ErrorInfo, MintError};
use crate::service::{CommonResult, FreeTokenService};
use crate::state::State;
use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk::api;
use ic_cdk_macros::*;
use log::{debug, logger, LevelFilter};
use std::panic;
use yansi::Paint;

#[init]
#[candid_method(init)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::clone_on_copy)]
fn canister_init(mintable: Principal, amount: Nat, unlimited_users: Option<Vec<Principal>>) {
    let service = FreeTokenService::default();
    service.init(&mintable, amount, unlimited_users);
    log::set_logger(&ICLogger);
    log::set_max_level(LevelFilter::Trace);
    panic::set_hook(Box::new(|data| {
        let message = format!("{}", data);
        api::print(Paint::red(message).to_string());
    }));
}

#[update(name = "receive_free_token")]
#[candid_method(update)]
async fn receive_free_token(key: String) -> BooleanResult {
    let caller = api::caller();
    let service = FreeTokenService::default();

    if check_is_unlimited_user(&caller) {
        let result = service.unlimited_receive_free_token(&caller, &mintable).await;
        result.into()
    } else {
        let result = service.receive_free_token(&caller, &mintable).await;
        result.into()
    }
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

candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query)]
fn __export_did_tmp_() -> String {
    __export_service()
}
