use crate::canister_api::api_impl::{DFTApi, ICNamingApi};
use crate::canister_api::{IDFTApi, IICNamingApi, OperationResult};
use crate::permissions::{must_not_anonymous, ActorResult, ErrorInfo, MintError};
use crate::received_reward_store::ReceivesRewardRecord;
use crate::reward_store::{RewardCode, RewardPackage, RewardType};
use crate::state::{State, TransactionId, User, STATE};
use crate::TimeInNs;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{decode_args, encode_args, CandidType, Nat, Principal};
use ic_cdk::api::call::{call_raw, CallResult, RejectionCode};
use ic_cdk::trap;
use log::{debug, error, info, logger};
use serde::Deserialize;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::ptr::null;
use std::sync::Arc;

#[cfg(test)]
pub mod tests;

pub struct FreeTokenService {
    pub dft_api: Arc<dyn IDFTApi>,
    pub icnaming_api: Arc<dyn IICNamingApi>,
}

impl Default for FreeTokenService {
    fn default() -> Self {
        Self {
            dft_api: Arc::new(DFTApi::default()),
            icnaming_api: Arc::new(ICNamingApi::default()),
        }
    }
}

impl FreeTokenService {
    pub async fn receive_free_token(
        &self,
        user: &Principal,
        code: &RewardCode,
        time: TimeInNs,
    ) -> CommonResult<bool> {
        let user = &must_not_anonymous(user)?;
        let mut reward_record = STATE.with(|state| {
            state.is_able_receive(user, code)?;
            state.receive_reward(user, code, time)
        })?;
        self.send_reward(user, &mut reward_record, time).await;
        STATE.with(|state| {
            let mut received_reward_store = state.received_reward_record_store.borrow_mut();
            received_reward_store.update_received_reward_record(
                user.clone(),
                code.clone(),
                &reward_record,
            );
        });
        Ok(true)
    }
    pub fn get_rewards(&self) -> CommonResult<Vec<RewardPackage>> {
        STATE.with(|state| {
            let reward_store = state.reward_store.borrow();
            Ok(reward_store.)
        })
    }

    pub async fn add_reward(
        &self,
        user: &Principal,
        code: RewardCode,
        reward_package: RewardPackage,
        unlimited_users: Option<Vec<Principal>>,
    ) -> CommonResult<(bool)> {
        let user = &must_not_anonymous(user)?;
        STATE.with(|state| {
            let mut reward_store = state.reward_store.borrow_mut();
            let mut unlimited_user_store = state.unlimited_user_store.borrow_mut();
            reward_store.add_reward(code.clone(), reward_package);
            if let Some(unlimited_users) = unlimited_users {
                unlimited_user_store.add_unlimited_user(
                    code.clone(),
                    unlimited_users.iter().map(|u| User(u.clone())).collect(),
                );
            }
        });

        Ok(true)
    }

    pub async fn send_reward(
        &self,
        user: &User,
        reward_record: &mut ReceivesRewardRecord,
        time: TimeInNs,
    ) {
        let dft_api = &self.dft_api;
        let icnaming_api = &self.icnaming_api;
        let principal: Principal = user.0;
        for reward_type in reward_record.get_reward_type() {
            let result: ActorResult<bool> = match reward_type.clone() {
                RewardType::TokenMintRewardPackage { canister, amount } => {
                    let result = dft_api
                        .mint(&canister, &principal, Some(time), amount.clone())
                        .await;
                    match result {
                        Ok(..) => Ok(true),
                        Err(e) => Err(e),
                    }
                }
                RewardType::TokenTransferRewardPackage { canister, amount } => {
                    let result = dft_api
                        .transfer(
                            &canister,
                            None,
                            principal.to_text(),
                            amount.clone(),
                            Some(time),
                        )
                        .await;
                    match result {
                        Ok(..) => Ok(true),
                        Err(e) => Err(e),
                    }
                }
                RewardType::QuotaRewardPackage {
                    canister,
                    quota_type,
                    diff,
                } => {
                    icnaming_api
                        .transfer_quota(&canister, principal, quota_type.clone(), diff.clone())
                        .await
                }
            };
            match result {
                Ok(..) => (reward_record.set_reward_state_completed(&reward_type)),
                Err(e) => {
                    error!("send reward error: {:?}", e);
                }
            }
        }
    }
}

pub type CommonResult<T> = anyhow::Result<T, MintError>;
