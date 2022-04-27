use crate::permissions::{ActorResult, ErrorInfo};
use crate::state::STATE;
use async_trait::async_trait;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{decode_args, encode_args, CandidType, Nat, Principal};
use ic_cdk::api::call::{call_raw, CallResult, RejectionCode};
use ic_cdk::trap;
use log::{debug, error};
use serde::Deserialize;
use std::fmt::Debug;
use std::future::Future;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Default)]
pub struct DFTApi {}

#[async_trait]
pub trait IDFTApi {
    async fn mint(&self, user: &Principal, created_at: Option<u64>)
        -> ActorResult<OperationResult>;
}

#[async_trait]
impl IDFTApi for DFTApi {
    async fn mint(
        &self,
        user: &Principal,
        created_at: Option<u64>,
    ) -> ActorResult<OperationResult> {
        let canister = STATE.with(|state| state.free_settings.borrow().minter.clone());
        let free_amount = STATE.with(|state| state.free_settings.borrow().free_amount.clone());
        call_canister_as_actor_result(
            &canister,
            "mint",
            (user.to_text(), free_amount.clone(), created_at),
        )
        .await
    }
}
pub type TransactionId = String;
// #[derive(CandidType, Debug, Deserialize)]
// pub enum OperationResult {
//     Ok {
//         #[serde(rename = "txId")]
//         tx_id: TransactionId,
//         #[serde(rename = "blockHeight")]
//         block_height: Nat,
//         error: Option<ErrorInfo>,
//     },
//     Err(ErrorInfo),
// }

#[derive(CandidType, Debug, Clone, Deserialize, Default)]
pub struct OperationResult {
    #[serde(rename = "txId")]
    tx_id: TransactionId,
    #[serde(rename = "blockHeight")]
    block_height: Nat,
    error: Option<ErrorInfo>,
}

async fn call_canister_as_actor_result<T, TResult>(
    canister_id: &Principal,
    method: &str,
    args: T,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder + Debug,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    let result = call_core::<T, ActorResult<TResult>>(&canister_id, method, args, true).await;
    match result {
        Ok(result) => result,
        Err(error) => Err(ErrorInfo::from(error)),
    }
}

async fn call_core<T, TResult>(
    canister_id: &Principal,
    method: &str,
    args: T,
    logging: bool,
) -> Result<TResult, ErrorInfo>
where
    T: candid::utils::ArgumentEncoder + Debug,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    if logging {
        debug!("Calling {}::{} {:?}", canister_id, method, args);
    }
    let call_result: Result<(TResult,), (RejectionCode, String)> =
        call(*canister_id, method, args).await;
    if call_result.is_err() {
        let (code, message) = call_result.err().unwrap();
        let code_string = format!("{:?}", code);
        error!(
            "{}::{} failed with code {}: {}",
            canister_id, method, code_string, message
        );
        return Err(ErrorInfo {
            code: code as u32,
            message,
        });
    }
    let result = call_result.unwrap();
    if logging {
        debug!(
            "Call canister {} with method {} result: {:?}",
            canister_id, method, result
        );
    }
    Ok(result.0)
}

pub fn call<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    id: Principal,
    method: &str,
    args: T,
) -> impl Future<Output = CallResult<R>> {
    let args_raw = encode_args(args).expect("Failed to encode arguments.");
    let fut = call_raw(id, method, &args_raw, 0);
    async {
        let bytes = fut.await?;
        decode_args(&bytes).map_err(|err| trap(&format!("{:?}", err)))
    }
}
