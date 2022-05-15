use crate::canister_api::{IDFTApi, IICNamingApi, OperationResult};
use crate::permissions::{must_not_anonymous, ActorResult, ErrorInfo, MintError};
use crate::state::{TransactionId, STATE, State, User};
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
use crate::canister_api::api_impl::{DFTApi, ICNamingApi};
use crate::received_reward_store::ReceivesRewardRecord;
use crate::reward_store::{RewardCode, RewardPackage, RewardType};
use crate::TimeInNs;

#[cfg(test)]
mod tests;

pub struct FreeTokenService {
    pub dft_api: Arc<dyn IDFTApi>,
    pub icnaming_api: Arc<dyn IICNamingApi>,
}

impl Default for FreeTokenService {
    fn default() -> Self {
        Self {
            dft_api: Arc::new(DFTApi::default()),
            icnaming_api: Arc::new(ICNamingApi::new()),
        }
    }
}

impl FreeTokenService {
    pub async fn receive_free_token(&self, user: &Principal, code: &RewardCode, time: TimeInNs) -> CommonResult<bool> {
        let user = &must_not_anonymous(user)?;

        let mut reward_record = STATE.with(|state| {
            state.is_able_receive(user, code);
            state.receive_reward(user, code, time)
        })?;


        self.send_reward(user, &mut reward_record, time).await;


        STATE.with(|state| {
            let mut received_reward_store = state.received_reward_record_store.borrow_mut();
            received_reward_store.update_received_reward_record(user.clone(), code.clone(), &reward_record);
        });
        Ok(true)
        // if result.is_ok() {
        //     STATE.with(|state| {
        //         let mut records = state.free_records.borrow_mut();
        //         let record = FreeTokenRecord::new(
        //             setting.mintable,
        //             setting.free_amount,
        //             Option::Some(now),
        //             user.clone(),
        //         );
        //         records
        //             .entry(mintable.clone())
        //             .or_insert_with(HashMap::new)
        //             .insert(user.clone(), record);
        //     });
        // }
        // match result {
        //     Ok(..) => Ok(true),
        //     Err(e) => {
        //         error!("mint error: {:?}", e);
        //         Err(MintError::from(e).into())
        //     }
        // }
    }


    pub async fn send_reward(&self, user: &User, reward_record: &mut ReceivesRewardRecord, time: TimeInNs) {
        let dft_api = &self.dft_api;
        let icnaming_api = &self.icnaming_api;
        let principal: Principal = user.0;
        for reward_type in reward_record.get_reward_type() {
            let result: ActorResult<bool> = match reward_type.clone() {
                RewardType::TokenMintRewardPackage { canister, amount } => {
                    let result = dft_api.mint(&canister, &principal, Some(time), amount.clone()).await;
                    match result {
                        Ok(..) => Ok(true),
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
                RewardType::TokenTransferRewardPackage { canister, amount } => {
                    let result = dft_api.transfer(&canister, None, principal.to_text(), amount.clone(), Some(time)).await;
                    match result {
                        Ok(..) => Ok(true),
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
                RewardType::QuotaRewardPackage { canister, quota_type, diff } => {
                    icnaming_api.transfer_quota(&canister, principal, quota_type.clone(), diff.clone()).await
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
