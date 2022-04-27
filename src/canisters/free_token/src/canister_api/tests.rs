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
    async fn mint(&self, user: &Principal, created_at: Option<u64>)
    -> ActorResult<OperationResult>;

}
}

#[fixture]
pub fn mock_dft_api() -> MockDFTApi {
    MockDFTApi::default()
}
