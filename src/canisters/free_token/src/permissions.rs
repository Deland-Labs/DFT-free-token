use crate::canister_api::OperationResult;
use crate::constants::FREE_TOKEN_PRINCIPAL_NAME_ADMIN;
use crate::state::{User, STATE};
use candid::{CandidType, Deserialize, Principal};
use std::borrow::Borrow;
use thiserror::Error;

pub fn must_not_anonymous(caller: &Principal) -> DexServiceResult<User> {
    if *caller == Principal::anonymous() {
        return Err(FreeTokenError::Unauthorized);
    }
    Ok(User(caller.clone()))
}

pub fn must_be_system_owner(caller: &Principal) -> DexServiceResult<()> {
    must_not_anonymous(caller)?;
    if !is_admin(caller) {
        return Err(FreeTokenError::Unauthorized);
    }
    Ok(())
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Error)]
pub enum FreeTokenError {
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

impl FreeTokenError {
    pub(crate) fn code(&self) -> u32 {
        match self {
            FreeTokenError::Unknown => 1,
            FreeTokenError::RemoteError(_) => 2,
            FreeTokenError::Unauthorized => 3,
            FreeTokenError::RewardAlreadyReceived => 4,
            FreeTokenError::CanisterCallError { .. } => 5,
            FreeTokenError::UnknownMintable => 6,
            FreeTokenError::RewardIncomplete => 7,
            FreeTokenError::RewardCodeNotAvailable => 8,
        }
    }
}

pub fn get_error_code(error: FreeTokenError) -> ErrorInfo {
    ErrorInfo {
        code: error.code(),
        message: error.to_string(),
    }
}

pub type DexServiceResult<T> = anyhow::Result<T, FreeTokenError>;

impl From<FreeTokenError> for ErrorInfo {
    fn from(error: FreeTokenError) -> Self {
        get_error_code(error)
    }
}

impl From<ErrorInfo> for FreeTokenError {
    fn from(error: ErrorInfo) -> Self {
        FreeTokenError::RemoteError(error)
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

pub fn is_admin(user: &Principal) -> bool {
    let admin = Principal::from_text(FREE_TOKEN_PRINCIPAL_NAME_ADMIN).unwrap();
    user == &admin
}
