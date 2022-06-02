use crate::canister_api::*;
use crate::permissions::*;
use crate::service::*;
use async_trait::async_trait;
use candid::Nat;
use mockall::{mock, predicate::*};
use rstest::*;
mock! {
    #[derive(Debug)]
    pub DFTApi {
    }

    #[async_trait]
    impl IDFTApi for DFTApi {
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

}

mock! {

   #[derive(Debug)]
    pub ICNamingApi {
    }

    #[async_trait]
    impl IICNamingApi for ICNamingApi {
        async fn transfer_quota(
            &self,
            canister: &Principal,
            to: Principal,
            quota_type: QuotaType,
            diff: u32,
        ) -> ActorResult<bool>;

        async fn batch_transfer_quota(&self, canister: &Principal, request: BatchTransferRequest) -> ActorResult<bool>;

        async fn transfer_from_quota(
            &self,
            canister: &Principal,
            request: TransferFromQuotaRequest,
        ) -> ActorResult<bool>;
        async fn approve(&self, canister: &Principal, name: String, to: Principal) -> ActorResult<bool>;

        async fn transfer(&self, canister: &Principal, name: String, new_owner: Principal) -> ActorResult<bool>;

        async fn transfer_from(&self, canister: &Principal, name: String) -> ActorResult<bool>;
    }

}



#[fixture]
pub fn mock_dft_api() -> MockDFTApi {
    MockDFTApi::default()
}

#[fixture]
pub fn mock_icnaming_api() -> MockICNamingApi {
    MockICNamingApi::default()
}
