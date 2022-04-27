use crate::canister_api::{DFTApi, IDFTApi, OperationResult};
use crate::ic_api::ic_now;
use crate::permissions::{
    must_not_anonymous, must_not_have_received, ActorResult, ErrorInfo, MintError,
};
use crate::state::{FreeTokenRecord, TransactionId, STATE};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{decode_args, encode_args, CandidType, Nat, Principal};
use ic_cdk::api::call::{call_raw, CallResult, RejectionCode};
use ic_cdk::trap;
use log::{debug, error, info, logger};
use serde::Deserialize;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::future::Future;
use std::ptr::null;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub struct FreeTokenService {
    pub dft_api: Arc<dyn IDFTApi>,
}

impl Default for FreeTokenService {
    fn default() -> Self {
        Self {
            dft_api: Arc::new(DFTApi::default()),
        }
    }
}

impl FreeTokenService {
    pub async fn receive_free_token(&self, user: &Principal) -> CommonResult<bool> {
        must_not_anonymous(user)?;
        must_not_have_received(user)?;
        let dft_api = &self.dft_api;
        let now = ic_now();
        let result = dft_api.mint(user, Option::Some(now)).await;
        debug!("mint result: {:?}", result);
        if result.is_ok() {
            STATE.with(|state| {
                let mut records = state.free_records.borrow_mut();
                let settings = state.free_settings.borrow();
                let record = FreeTokenRecord::new(
                    settings.minter.clone(),
                    settings.free_amount.clone(),
                    Option::Some(now),
                    user.clone(),
                );
                records.insert(user.clone(), record);
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

    pub async fn unlimited_receive_free_token(&self, user: &Principal) -> CommonResult<bool> {
        let dft_api = &self.dft_api;
        let result = dft_api.mint(user, None).await;
        debug!("mint result: {:?}", result);
        match result {
            Ok(..) => Ok(true),
            Err(e) => {
                error!("mint error: {:?}", e);
                Err(MintError::from(e).into())
            }
        }
    }
    pub fn init(&self, minter: &Principal, amount: Nat, unlimited_users: Option<Vec<Principal>>) {
        STATE.with(|state| {
            let mut free_settings = state.free_settings.borrow_mut();
            let mut unlimited_users_settings = state.unlimited_users.borrow_mut();
            free_settings.minter = minter.clone();
            free_settings.free_amount = amount;
            if let Some(unlimited_users) = unlimited_users {
                unlimited_users_settings.extend(unlimited_users);
            } else {
                unlimited_users_settings.clear();
            }
        });
    }
}

pub type CommonResult<T> = anyhow::Result<T, MintError>;
