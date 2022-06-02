use crate::permissions::{ActorResult, ErrorInfo};
use crate::state::{STATE};
use async_trait::async_trait;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{decode_args, encode_args, CandidType, Nat, Principal};
use ic_cdk::api::call::{call_raw, CallResult, RejectionCode};
use ic_cdk::trap;
use log::{debug, error};
use serde::Deserialize;
use std::fmt::{Debug, Display};
use std::future::Future;
use crate::reward_store::{QuotaType, RewardPackage, RewardType};
use crate::TimeInNs;

#[cfg(test)]
pub mod tests;
pub mod api_impl;

#[async_trait]
pub trait IICNamingApi {
    async fn transfer_quota(
        &self,
        canister: &Principal,
        to: Principal,
        quota_type: QuotaType,
        diff: u32,
    ) -> ActorResult<bool>;

    async fn batch_transfer_quota(&self, canister: &Principal, request: BatchTransferRequest) -> ActorResult<bool>;

    async fn transfer_from_quota(&self, canister: &Principal, request: TransferFromQuotaRequest)
                                 -> ActorResult<bool>;

    async fn approve(&self, canister: &Principal, name: String, to: Principal) -> ActorResult<bool>;

    async fn transfer(&self, canister: &Principal, name: String, new_owner: Principal) -> ActorResult<bool>;

    async fn transfer_from(&self, canister: &Principal, name: String) -> ActorResult<bool>;
}


#[async_trait]
pub trait IDFTApi {
    async fn mint(&self, canister: &Principal, user: &Principal, created_at: Option<TimeInNs>, value: Nat)
                  -> ActorResult<OperationResult>;
    async fn transfer(
        &self,
        canister: &Principal,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        created_at: Option<TimeInNs>,
    ) -> ActorResult<OperationResult>;
    async fn approve(
        &self,
        canister: &Principal,
        owner_sub_account: Option<Subaccount>,
        spender: String,
        value: Nat,
        created_at: Option<TimeInNs>,
    ) -> ActorResult<OperationResult>;
}

#[derive(CandidType, Debug, Clone, Deserialize, Default)]
pub struct OperationResult {
    #[serde(rename = "txId")]
    tx_id: TransactionId,
    #[serde(rename = "blockHeight")]
    block_height: Nat,
    error: Option<ErrorInfo>,
}

impl From<OperationResult> for bool {
    fn from(result: OperationResult) -> bool {
        result.error.is_none()
    }
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
        Err(error) => Err(error),
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
    let call_result: Result<(TResult, ), (RejectionCode, String)> =
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
) -> impl Future<Output=CallResult<R>> {
    let args_raw = encode_args(args).expect("Failed to encode arguments.");
    let fut = call_raw(id, method, &args_raw, 0);
    async {
        let bytes = fut.await?;
        decode_args(&bytes).map_err(|err| trap(&format!("{:?}", err)))
    }
}

#[derive(CandidType, Debug, Deserialize, Clone)]
pub struct TransferFromQuotaRequest {
    pub from: Principal,
    pub to: Principal,
    pub quota_type: QuotaType,
    pub diff: u32,
}

impl Display for TransferFromQuotaRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TransferFromQuotaRequest {{ from: {}, to: {}, quota_type: {}, diff: {} }}",
            self.from, self.to, self.quota_type, self.diff
        )
    }
}

impl TransferFromQuotaRequest {
    pub fn new(from: Principal, to: Principal, quota_type: QuotaType, diff: u32) -> Self {
        Self {
            from,
            to,
            quota_type,
            diff,
        }
    }
}

#[derive(Debug, Deserialize, CandidType)]
pub struct BatchTransferRequest {
    pub items: Vec<TransferQuotaDetails>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct TransferQuotaDetails {
    pub to: Principal,
    pub quota_type: QuotaType,
    pub diff: u32,
}


pub type Subaccount = [u8; 32];
pub type TransactionId = String;