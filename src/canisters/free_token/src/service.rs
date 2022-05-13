use crate::canister_api::{DFTApi, IDFTApi, IICNamingApi, OperationResult};
use crate::ic_api::ic_now;
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
use crate::reward_store::{RewardCode, RewardType};

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
    pub async fn receive_free_token(&self, user: &Principal, code: &RewardCode) -> CommonResult<bool> {
        let user = must_not_anonymous(user)?;
        let now = ic_now();

        let reward_package = STATE.with(|state| {
            let state = state.borrow();
            state.is_able_receive(user.clone(), code);
            state.receive_reward(user.clone(), code, Some(now))?
        });

        let dft_api = &self.dft_api;
        for reward_type in reward_package.reward_types() {
            let result = match reward_type {
                RewardType::TokenMintRewardPackage { canister, amount } => {
                    dft_api.mint(user.into(), Option::Some(now), reward_package.clone()).await
                },
                RewardType::TokenBurnRewardPackage { canister, amount } => {
                    dft_api.burn(user.into(), Option::Some(now), reward_package.clone()).await
                }

            }
            debug!("mint result: {:?}", result);
            if result.is_ok() {
                STATE.with(|state| {
                    let mut records = state.free_records.borrow_mut();
                    let record = FreeTokenRecord::new(
                        setting.mintable,
                        setting.free_amount,
                        Option::Some(now),
                        user.clone(),
                    );
                    records
                        .entry(mintable.clone())
                        .or_insert_with(HashMap::new)
                        .insert(user.clone(), record);
                });
            }
            match result {
                Ok(..) => Ok(true),
                Err(e) => {
                    error!("mint error: {:?}", e);
                    Err(MintError::from(e).into())
                }
            }
        }

        pub async fn unlimited_receive_free_token(&self, user: &Principal, mintable: &Principal) -> CommonResult<bool> {
            let dft_api = &self.dft_api;
            let setting = get_mintable(mintable)?;
            let now = ic_now();
            let result = dft_api.mint(user, Some(now), setting).await;
            debug!("mint result: {:?}", result);
            match result {
                Ok(..) => Ok(true),
                Err(e) => {
                    error!("mint error: {:?}", e);
                    Err(MintError::from(e).into())
                }
            }
        }
        pub fn init(&self, mintable: &Principal, amount: Nat, unlimited_users: Option<Vec<Principal>>) {
            STATE.with(|state| {
                let mut free_settings = state.free_settings.borrow_mut();
                let mut unlimited_users_settings = state.unlimited_users.borrow_mut();
                free_settings.push(FreeSetting::new(amount, mintable.clone()));
                if let Some(unlimited_users) = unlimited_users {
                    unlimited_users_settings.extend(unlimited_users);
                } else {
                    unlimited_users_settings.clear();
                }
            });
        }
    }

    pub type CommonResult<T> = anyhow::Result<T, MintError>;
