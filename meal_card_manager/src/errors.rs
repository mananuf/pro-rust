use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CampusMealSystemError {
    #[error("Student ID Error: {0}")]
    StudentIdError(String),
}

#[derive(Debug, Error, PartialEq)]
pub enum TransactionError {
    #[error("Transaction Error: {0}")]
    StudentNotFound(String),

    #[error("Transaction Error: {0}")]
    InsufficientFunds(u64),

    #[error("Transaction Error: {0}")]
    SuspendedAccount(String),
}
