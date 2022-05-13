use crate::actor::BooleanResult;
use crate::canister_api::tests::*;
use crate::canister_api::OperationResult;
use crate::service::FreeTokenService;
use candid::{Nat, Principal};
use rstest::*;
use std::sync::Arc;
use crate::state::FreeSetting;

// #[fixture]
// fn test_token_id() -> Principal {
//     Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
// }

#[fixture]
pub fn mock_user1() -> Principal {
    mock_user(1)
}

#[fixture]
pub fn mock_user2() -> Principal {
    mock_user(2)
}

#[fixture]
pub fn mock_user3() -> Principal {
    mock_user(2)
}

#[fixture]
pub fn mock_user4() -> Principal {
    mock_user(2)
}

#[fixture]
pub fn unlimited_users(mock_user3: Principal, mock_user4: Principal) -> Vec<Principal> {
    vec![mock_user3, mock_user4]
}

#[fixture]
pub fn mock_mitable1() -> Principal {
    mock_user(1000)
}

#[fixture]
pub fn mock_mitable2() -> Principal {
    mock_user(2000)
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
    mock_user1: Principal,
    mintable_setting: FreeSetting,
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
        unlimited_users: Vec<Principal>,
        service: FreeTokenService,
        mock_mitable: Principal,
    ) {
        service.init(&mock_mitable, Nat::from(100u32), Some(unlimited_users));
        let res = service.receive_free_token(&mock_user1, &mock_mitable).await;
        assert_eq!(res.is_ok(), true);
    }

    #[rstest]
    async fn test_received_again_free_token(
        mock_user1: Principal,
        unlimited_users: Vec<Principal>,
        service: FreeTokenService,
        mock_mitable: Principal,
    ) {
        let unlimited_users = vec![mock_user3, mock_user4];
        service.init(&mock_mitable, Nat::from(100u32), Some(unlimited_users));
        let res = service.receive_free_token(&mock_user1).await;
        assert_eq!(res.is_ok(), true);
        let res = service.receive_free_token(&mock_user1).await;
        println!("{:?}", res.clone().unwrap_err());
        assert_eq!(res.clone().unwrap_err(), MintError::AlreadyReceived);
        assert_eq!(res.is_err(), true);
    }
}
