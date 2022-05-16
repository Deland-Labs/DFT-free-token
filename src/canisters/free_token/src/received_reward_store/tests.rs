use crate::reward_store::RewardPackage;
use rstest::*;
use crate::service::tests::*;
use super::*;

#[fixture]
fn empty_received_reward_record_manager() -> ReceivedRewardRecordStore {
    ReceivedRewardRecordStore::new()
}

#[fixture]
fn received_reward_record_manager_with_one_completed(
    mut _empty_received_reward_record_manager: ReceivedRewardRecordStore,
    mock_user1: Principal,
    mock_user2: Principal,
    mock_user3: Principal,
    reward_package_store_1: HashMap<RewardCode, RewardPackage>,
    mock_now: u64,
) -> ReceivedRewardRecordStore {
    let mut manager = _empty_received_reward_record_manager;
    let mut reward_record_hash = HashMap::new();
    //get first value
    let record_types = reward_package_store_1.values().next().unwrap().reward_types();
    let code = reward_package_store_1.keys().next().unwrap();
    for reward_type in record_types {
        reward_record_hash.insert(reward_type.clone(), ReceivesRewardRecordState::Completed);
    }
    let reward_record = ReceivesRewardRecord::new(reward_record_hash, TimeInNs(mock_now));
    // act
    manager.add_received_reward_record(
        User(mock_user1.clone()),
        code.clone(),
        reward_record,
    );
    manager
}

#[rstest]
fn test_add_one_completed(
    mut received_reward_record_manager_with_one_completed: ReceivedRewardRecordStore,
    mock_user1: Principal,
    reward_package_store_1: HashMap<RewardCode, RewardPackage>,
    mock_now: u64,
) {
    let mut manager = received_reward_record_manager_with_one_completed;
    let record_types = reward_package_store_1.values().next().unwrap().reward_types();
    let code = reward_package_store_1.keys().next().unwrap();
    let mut reward_record_hash = HashMap::new();
    for reward_type in record_types {
        reward_record_hash.insert(reward_type.clone(), ReceivesRewardRecordState::Completed);
    }
    let reward_record = ReceivesRewardRecord::new(reward_record_hash, TimeInNs(mock_now));
    // act
    manager.add_received_reward_record(
        User(mock_user1.clone()),
        code.clone(),
        reward_record,
    );
    let result = manager.get_received_reward_record(&code, &User(mock_user1));
    // assert

}