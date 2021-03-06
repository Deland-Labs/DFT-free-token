use crate::actor::BooleanResult;
use crate::canister_api::tests::*;
use crate::canister_api::OperationResult;
use crate::reward_store::{QuotaType, RewardCode, RewardPackage, RewardType};
use crate::service::FreeTokenService;
use crate::state::{User, STATE};
use crate::TimeInNs;
use candid::{Nat, Principal};
use log::LevelFilter;
use pretty_env_logger::env_logger;
use rstest::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::Arc;

pub fn mock_user(index: u32) -> Principal {
    let mut principal_bytes = vec![0u8; 29];
    // The first four bytes are the index.
    principal_bytes[0..4].copy_from_slice(&index.to_be_bytes());
    Principal::from_slice(&principal_bytes)
}

pub fn init_test_logger() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .is_test(true)
        .try_init();
}

#[fixture]
pub fn init_test() {
    init_test_logger();
}

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
pub fn dft_transfer_canister() -> Principal {
    mock_user(1000)
}

#[fixture]
pub fn dft_mint_canister() -> Principal {
    mock_user(1000)
}

#[fixture]
pub fn icnaming_canister() -> Principal {
    mock_user(2000)
}

#[fixture]
pub fn mock_now() -> u64 {
    15_000_000_000
}

#[fixture]
pub fn mock_reward_quota_1() -> RewardType {
    RewardType::QuotaRewardPackage {
        quota_type: QuotaType::LenGte(5),
        canister: icnaming_canister(),
        diff: 7,
    }
}

#[fixture]
pub fn mock_reward_mint_1() -> RewardType {
    RewardType::TokenMintRewardPackage {
        canister: dft_mint_canister(),
        amount: Nat::from(1000),
    }
}

#[fixture]
pub fn mock_reward_transfer_1() -> RewardType {
    RewardType::TokenTransferRewardPackage {
        canister: dft_transfer_canister(),
        amount: Nat::from(1000),
    }
}

#[fixture]
pub fn reward_package_type() -> Vec<RewardType> {
    let mut result = vec![];
    result.push(mock_reward_quota_1());
    result.push(mock_reward_mint_1());
    result.push(mock_reward_transfer_1());

    return result;
}

#[fixture]
pub fn reward_package_store_1() -> HashMap<RewardCode, RewardPackage> {
    let mut hashmap = HashMap::new();
    hashmap
        .entry(RewardCode(String::from("reward_code_1")))
        .or_insert_with(|| RewardPackage::new(reward_package_type()));

    return hashmap;
}

#[fixture]
fn service(
    init_test: (),
    mock_user1: Principal,
    mock_user2: Principal,
    mock_user3: Principal,
    mock_user4: Principal,
    mut mock_dft_api: MockDFTApi,
    mut mock_icnaming_api: MockICNamingApi,
    reward_package_store_1: HashMap<RewardCode, RewardPackage>,
    mock_reward_quota_1: RewardType,
    mock_reward_mint_1: RewardType,
    mock_reward_transfer_1: RewardType,
    mock_now: u64,
) -> FreeTokenService {
    STATE.with(|s| {
        let mut reward_store = s.reward_store.borrow_mut();
        let mut unlimited_user_store = s.unlimited_user_store.borrow_mut();
        for (code, reward_package) in reward_package_store_1.into_iter() {
            reward_store.add_reward(code.clone(), reward_package);
            unlimited_user_store
                .add_unlimited_user(code.clone(), vec![User(mock_user4), User(mock_user3)]);
        }
    });
    let mut service = FreeTokenService::default();
    mock_dft_api
        .expect_mint()
        .returning(move |canister, user, created_at, value| {
            if let RewardType::TokenMintRewardPackage {
                canister: canister_expect,
                amount: amount_expect,
            } = mock_reward_mint_1.clone()
            {
                //assert_eq!(user, &mock_user1);
                assert_eq!(created_at, Some(TimeInNs(mock_now)));
                assert_eq!(value, amount_expect);
                assert_eq!(canister, &canister_expect);
                Ok(OperationResult::default())
            } else {
                panic!("Unexpected reward type");
            }
        });
    mock_dft_api.expect_transfer().returning(
        move |canister, from_sub_account, user, value, created_at| {
            if let RewardType::TokenTransferRewardPackage {
                canister: canister_expect,
                amount: amount_expect,
            } = mock_reward_transfer_1.clone()
            {
                //assert_eq!(user, mock_user1.to_text());
                assert_eq!(created_at, Some(TimeInNs(mock_now)));
                assert_eq!(value, amount_expect);
                assert_eq!(canister, &canister_expect);
                Ok(OperationResult::default())
            } else {
                panic!("Unexpected reward type");
            }
        },
    );
    mock_icnaming_api
        .expect_transfer_quota()
        .returning(move |canister, user, quota_type, diff| {
            if let RewardType::QuotaRewardPackage {
                canister: canister_expect,
                quota_type: quota_type_expect,
                diff: diff_expect,
            } = mock_reward_quota_1.clone()
            {
                //assert_eq!(user, mock_user1);
                assert_eq!(quota_type, quota_type_expect);
                assert_eq!(diff, diff_expect);
                assert_eq!(canister, &canister_expect);
                Ok(true)
            } else {
                panic!("Unexpected reward type");
            }
        });
    service.dft_api = Arc::new(mock_dft_api);
    service.icnaming_api = Arc::new(mock_icnaming_api);
    service
}

mod ensure_received_reward_package_1 {
    use super::*;
    use crate::permissions::MintError;
    use crate::state::User;
    use crate::TimeInNs;

    #[rstest]
    async fn test_ensure_received_reward_package_1(
        mock_user1: Principal,
        service: FreeTokenService,
        reward_package_store_1: HashMap<RewardCode, RewardPackage>,
        mock_now: u64,
    ) {
        let record_types = reward_package_store_1
            .values()
            .next()
            .unwrap()
            .reward_types();
        let code = reward_package_store_1.keys().next().unwrap();
        let result = service
            .receive_free_token(&mock_user1, &code, TimeInNs(mock_now))
            .await;
        match result {
            Ok(b) => {
                assert_eq!(b, true);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[rstest]
    async fn test_ensure_received_reward_package_1_twice_should_failed(
        mock_user1: Principal,
        service: FreeTokenService,
        reward_package_store_1: HashMap<RewardCode, RewardPackage>,
        mock_now: u64,
    ) {
        let record_types = reward_package_store_1
            .values()
            .next()
            .unwrap()
            .reward_types();
        let code = reward_package_store_1.keys().next().unwrap();
        let result = service
            .receive_free_token(&mock_user1, &code, TimeInNs(mock_now))
            .await;
        match result {
            Ok(res) => {
                assert_eq!(res, true);
            }
            Err(e) => panic!("{:?}", e),
        }
        let result = service
            .receive_free_token(&mock_user1, &code, TimeInNs(mock_now))
            .await;
        match result {
            Ok(res) => {
                panic!("should failed");
            }
            Err(e) => assert_eq!(e, MintError::RewardAlreadyReceived),
        }
    }

    #[rstest]
    async fn test_ensure_received_reward_package_1_unlimited_user(
        mock_user3: Principal,
        service: FreeTokenService,
        reward_package_store_1: HashMap<RewardCode, RewardPackage>,
        mock_now: u64,
    ) {
        let record_types = reward_package_store_1
            .values()
            .next()
            .unwrap()
            .reward_types();
        let code = reward_package_store_1.keys().next().unwrap();
        let result = service
            .receive_free_token(&mock_user3, code, TimeInNs(mock_now))
            .await;
        match result {
            Ok(res) => {
                assert_eq!(res, true);
            }
            Err(e) => panic!("{:?}", e),
        }
        let result = service
            .receive_free_token(&mock_user3, code, TimeInNs(mock_now))
            .await;
        match result {
            Ok(res) => {
                assert_eq!(res, true);
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
