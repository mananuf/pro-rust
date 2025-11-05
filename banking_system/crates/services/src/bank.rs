use std::sync::Arc;

use bank_core::{account::{AccountId, AccountRepository, Money, Transaction}, customer::CustomerId, errors::AppError};

pub struct Bank<R: AccountRepository> {
    pub repo: Arc<R>,
}

impl<R: AccountRepository> Bank<R> {
    fn create_account(&self, owner: CustomerId) -> Result<AccountId, AppError> {
        
        todo!()
    }

    fn process(&self, account_id: AccountId, txn: Transaction) -> Result<Money, AppError> {
        todo!()
    }

    fn transfer(&self, from: AccountId, to: AccountId, amount: Money) -> Result<(), AppError> {
        todo!()
    }
}