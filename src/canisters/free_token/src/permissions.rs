use crate::state::STATE;
use candid::{CandidType, Deserialize, Principal};
use std::borrow::Borrow;
use thiserror::Error;

pub fn must_not_have_received(caller: &Principal) -> Result<(), MintError> {
    STATE.with(|state| {
        if state.borrow().free_records.borrow().get(caller).is_some() {
            Err(MintError::AlreadyReceived)
        } else {
            Ok(())
        }
    })
}
pub fn check_is_unlimited_user(caller: &Principal) -> bool {
    STATE.with(|state| {
        if state.borrow().unlimited_users.borrow().contains(caller) {
            true
        } else {
            false
        }
    })
}

pub fn must_not_anonymous(caller: &Principal) -> DexServiceResult<()> {
    if *caller == Principal::anonymous() {
        return Err(MintError::Unauthorized);
    }
    Ok(())
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum MintError {
    #[error("there is a unknown error raised")]
    Unknown,
    #[error("error from remote, {0:?}")]
    RemoteError(ErrorInfo),
    #[error("canister call error, rejected by {rejection_code:?}")]
    CanisterCallError {
        rejection_code: String,
        message: String,
    },
    #[error("Unauthorized, please login first")]
    Unauthorized,
    #[error("Already received, can not receive again")]
    AlreadyReceived,
}

impl MintError {
    pub(crate) fn code(&self) -> u32 {
        match self {
            MintError::Unknown => 1,
            MintError::RemoteError(_) => 2,
            MintError::Unauthorized => 3,
            MintError::AlreadyReceived => 4,
            MintError::CanisterCallError { .. } => 7,
        }
    }
}
pub fn get_error_code(error: MintError) -> ErrorInfo {
    ErrorInfo {
        code: error.code(),
        message: error.to_string(),
    }
}

pub type DexServiceResult<T> = anyhow::Result<T, MintError>;

impl From<MintError> for ErrorInfo {
    fn from(error: MintError) -> Self {
        get_error_code(error)
    }
}

impl From<ErrorInfo> for MintError {
    fn from(error: ErrorInfo) -> Self {
        MintError::RemoteError(error)
    }
}

pub type ActorResult<T> = Result<T, ErrorInfo>;

/// Error information
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize)]
pub struct ErrorInfo {
    /// Error code
    pub code: u32,
    /// Error message
    pub message: String,
}
