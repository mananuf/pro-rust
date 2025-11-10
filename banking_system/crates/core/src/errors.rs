use thiserror::Error;

use crate::account::AccountId;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Repo error: {0}")]
    Repo(#[from] RepoError),
}

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("deposit error: {0}")]
    ClosedAccount(String),
    #[error("deposit error: {0}")]
    NegativeAmount(String),
    #[error("deposit error: {0}")]
    FrozenAccount(String),
    #[error("withdrawal error: {0}")]
    InsufficientFunds(String),
    #[error("transaction error: {0}")]
    Unsupported(String),
    #[error("Account {0} NOT FOUND")]
    AccountNotFound(AccountId),
    #[error("Transaction failed: cannot transfer to self")]
    TransferToSelf
}

#[derive(Debug, Error, PartialEq)]
pub enum RepoError {
    #[error("Lock Error: Lock poisened")]
    LockPoisened,
}
