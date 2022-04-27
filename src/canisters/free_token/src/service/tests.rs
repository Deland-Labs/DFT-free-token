use crate::actor::BooleanResult;
use crate::canister_api::tests::*;
use crate::canister_api::OperationResult;
use crate::service::FreeTokenService;
use candid::{Nat, Principal};
use rstest::*;
use std::sync::Arc;

#[fixture]
fn test_token_id() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}
#[fixture]
pub fn mock_user1() -> Principal {
    mock_user(1)
}

pub fn mock_user(index: u32) -> Principal {
    let mut principal_bytes = vec![0u8; 29];
    // The first four bytes are the index.
    principal_bytes[0..4].copy_from_slice(&index.to_be_bytes());
    Principal::from_slice(&principal_bytes)
}

#[fixture]
fn service(
    mut mock_dft_api: MockDFTApi,
    test_token_id: Principal,
    mock_user1: Principal,
) -> FreeTokenService {
    let mut service = FreeTokenService::default();

    let _ctx = mock_dft_api
        .expect_mint()
        .returning(move |principal, created_at| {
            assert_eq!(principal, &mock_user1);
            assert_eq!(created_at, None);
            Ok(OperationResult::default())
        });
    service.dft_api = Arc::new(mock_dft_api);
    service
}

mod receive_free_token {
    use super::*;
    use crate::permissions::MintError;
    use mockall::Any;

    #[rstest]
    async fn test_receive_free_token(
        mock_user1: Principal,
        service: FreeTokenService,
        test_token_id: Principal,
    ) {
        service.init(&test_token_id, Nat::from(100u32));
        let res = service.receive_free_token(&mock_user1).await;
        assert_eq!(res.is_ok(), true);
    }

    #[rstest]
    async fn test_received_again_free_token(
        mock_user1: Principal,
        service: FreeTokenService,
        test_token_id: Principal,
    ) {
        service.init(&test_token_id, Nat::from(100u32));
        let res = service.receive_free_token(&mock_user1).await;
        assert_eq!(res.is_ok(), true);
        let res = service.receive_free_token(&mock_user1).await;
        println!("{:?}", res.clone().unwrap_err());
        assert_eq!(res.clone().unwrap_err(), MintError::AlreadyReceived);
        assert_eq!(res.is_err(), true);
    }
}
