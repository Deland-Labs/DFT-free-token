use crate::state::{STATE, User};
use candid::{CandidType, Deserialize, Principal};
use std::borrow::Borrow;
use thiserror::Error;
use crate::canister_api::OperationResult;


pub fn must_not_anonymous(caller: &Principal) -> DexServiceResult<User> {
    if *caller == Principal::anonymous() {
        return Err(MintError::Unauthorized);
    }
    Ok(User(caller.clone()))
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
    RewardAlreadyReceived,
    #[error("Unknown mintable")]
    UnknownMintable,
    #[error("Reward incomplete")]
    RewardIncomplete,
    #[error("Reward code not available")]
    RewardCodeNotAvailable,
}

impl MintError {
    pub(crate) fn code(&self) -> u32 {
        match self {
            MintError::Unknown => 1,
            MintError::RemoteError(_) => 2,
            MintError::Unauthorized => 3,
            MintError::RewardAlreadyReceived => 4,
            MintError::CanisterCallError { .. } => 5,
            MintError::UnknownMintable => 6,
            MintError::RewardIncomplete => 7,
            MintError::RewardCodeNotAvailable => 8,
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
