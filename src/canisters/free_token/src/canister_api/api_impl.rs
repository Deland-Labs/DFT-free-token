use super::*;
use crate::canister_api::{
    call, call_canister_as_actor_result, BatchTransferRequest, IDFTApi, IICNamingApi,
    OperationResult, Subaccount, TransferFromQuotaRequest,
};
use crate::permissions::{ActorResult, ErrorInfo};
use crate::reward_store::QuotaType;
use candid::{Nat, Principal};

#[derive(Debug, Default)]
pub struct ICNamingApi {}

#[async_trait]
impl IICNamingApi for ICNamingApi {
    async fn transfer_quota(
        &self,
        canister: &Principal,
        to: Principal,
        quota_type: QuotaType,
        diff: u32,
    ) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "transfer_quota", (to, quota_type, diff)).await
    }

    async fn batch_transfer_quota(
        &self,
        canister: &Principal,
        request: BatchTransferRequest,
    ) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "batch_transfer_quota", (request,)).await
    }

    async fn transfer_from_quota(
        &self,
        canister: &Principal,
        request: TransferFromQuotaRequest,
    ) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "transfer_from_quota", (request,)).await
    }
    async fn approve(
        &self,
        canister: &Principal,
        name: String,
        to: Principal,
    ) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "approve", (name, to)).await
    }

    async fn transfer(
        &self,
        canister: &Principal,
        name: String,
        new_owner: Principal,
    ) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "transfer", (name, new_owner)).await
    }

    async fn transfer_from(&self, canister: &Principal, name: String) -> ActorResult<bool> {
        call_canister_as_actor_result(canister, "transfer_from", (name,)).await
    }
}

#[derive(Debug, Default)]
pub struct DFTApi {}

#[async_trait]
impl IDFTApi for DFTApi {
    async fn mint(
        &self,
        canister: &Principal,
        user: &Principal,
        created_at: Option<TimeInNs>,
        amount: Nat,
    ) -> ActorResult<OperationResult> {
        call_canister_as_actor_result(canister, "mint", (user.to_text(), amount, created_at)).await
    }

    async fn approve(
        &self,
        canister: &Principal,
        owner_sub_account: Option<Subaccount>,
        spender: String,
        value: Nat,
        created_at: Option<TimeInNs>,
    ) -> ActorResult<OperationResult> {
        call_canister_as_actor_result(
            canister,
            "approve",
            (owner_sub_account, spender, value, created_at),
        )
        .await
    }
    async fn transfer(
        &self,
        canister: &Principal,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        created_at: Option<TimeInNs>,
    ) -> ActorResult<OperationResult> {
        call_canister_as_actor_result(
            canister,
            "transfer",
            (from_sub_account, to, value, created_at),
        )
        .await
    }
}
