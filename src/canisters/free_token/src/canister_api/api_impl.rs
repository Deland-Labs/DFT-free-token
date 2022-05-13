use candid::{Nat, Principal};
use crate::canister_api::{BatchTransferRequest, call, call_canister_as_actor_result, call_canister_as_icns_result, IDFTApi, IICNamingApi, OperationResult, Subaccount, TransferFromQuotaRequest};
use crate::permissions::{ActorResult, ErrorInfo, ICNSActorResult};
use crate::reward_store::QuotaType;

pub struct ICNamingApi;

impl ICNamingApi {
    pub fn new() -> Self {
        ICNamingApi
    }
}

#[async_trait]
impl IICNamingApi for ICNamingApi {
    async fn transfer_quota(
        &self,
        canister: &Principal,
        to: Principal,
        quota_type: QuotaType,
        diff: u32,
    ) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister, "transfer_quota", (to, quota_type, diff)).await
    }

    async fn batch_transfer_quota(&self, canister: &Principal, request: BatchTransferRequest) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister_id, "batch_transfer_quota", (request, )).await
    }

    async fn transfer_from_quota(
        &self,
        canister: &Principal,
        request: TransferFromQuotaRequest,
    ) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister_id, "transfer_from_quota", (request, )).await
    }
    async fn approve(&self, canister: &Principal, name: String, to: Principal) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister_id, "approve", (name, to)).await
    }

    async fn transfer(&self, canister: &Principal, name: String, new_owner: Principal) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister_id, "transfer", (name, new_owner)).await
    }

    async fn transfer_from(&self, canister: &Principal, name: String) -> ICNSActorResult<bool> {
        call_canister_as_icns_result(canister_id, "transfer_from", (name, )).await
    }
}


#[derive(Debug, Default)]
pub struct DFTApi {}

#[async_trait]
impl IDFTApi for DFTApi {
    async fn mint(
        &self,
        user: &Principal,
        created_at: Option<u64>,
        canister: &Principal,
        amount: Nat,
    ) -> ActorResult<OperationResult> {
        call_canister_as_actor_result(
            canister,
            "mint",
            (user.to_text(), amount, created_at),
        )
            .await
    }

    async fn approve(
        &self,
        token_id: Principal,
        owner_sub_account: Option<Subaccount>,
        spender: String,
        value: Nat,
        created_at: Option<u64>,
    ) -> OperationResult {
        let result: Result<(OperationResult, ), _> = call(
            token_id,
            "approve",
            (owner_sub_account, spender, value, created_at),
        )
            .await;
        match result {
            Ok((result, )) => result,
            Err((error, msg)) => OperationResult::Err(ErrorInfo {
                message: msg,
                code: error as u32,
            }),
        }
    }
    async fn transfer(
        &self,
        token_id: Principal,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        created_at: Option<u64>,
    ) -> OperationResult {
        let result: Result<(OperationResult, ), _> = call(
            token_id,
            "transfer",
            (from_sub_account, to, value, created_at),
        )
            .await;
        match result {
            Ok((result, )) => result,
            Err((error, msg)) => OperationResult::Err(ErrorInfo {
                message: msg,
                code: error as u32,
            }),
        }
    }
}