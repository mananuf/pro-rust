use std::sync::{Arc, Mutex};

use bank_core::{
    account::{Account, AccountId, AccountRepository, Money, Transaction},
    customer::CustomerId,
    errors::{AppError, DomainError},
};


pub struct Bank<R: AccountRepository> {
    pub next_id: Mutex<AccountId>,
    pub repo: Arc<R>,
}

impl<R: AccountRepository> Bank<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Bank { next_id: Mutex::new(0), repo }
    }

    pub fn create_account(&mut self, owner: CustomerId) -> Result<AccountId, AppError> {
        let mut id = self.next_id.lock().unwrap();
        *id += 1;
        let account = Account::builder(*id, owner).build();

        self.repo.create(account)?;
        Ok(*id)
    }

    pub fn process(
        &self,
        account_id: AccountId,
        txn: Transaction
    ) -> Result<Money, AppError> {

        let mut account = self.repo.get(account_id)?
            .ok_or_else(|| DomainError::AccountNotFound(account_id))?;

        let result: Result<Money, AppError> = match &txn {
            Transaction::Transfer { to, amount } => {
                self.transfer(account_id, *to, *amount)?;
                Ok(account.balance)
            }
            _ => {
                let new_balance = account.apply_transaction(txn)?;
                self.repo.update(account.clone())?;
                Ok(new_balance)
            }
        };

        result.map_err(AppError::from)
    }

    fn transfer(
        &self,
        from: AccountId,
        to: AccountId,
        amount: Money
    ) -> Result<(), AppError> {

        if from == to {
            return Err(DomainError::TransferToSelf.into());
        }

        let mut src = self.repo.get(from)?
            .ok_or_else(|| DomainError::AccountNotFound(from))?;

        let mut dest = self.repo.get(to)?
            .ok_or_else(|| DomainError::AccountNotFound(to))?;

        src.withdraw(amount)?;

        dest.deposit(amount)?;

        self.repo.update(src)?;
        self.repo.update(dest)?;

        Ok(())
    }

}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use bank_core::{account::{AccountRepository, Money, Transaction}, customer::Customer};
    use bank_infra::storage::InMemoryRepo;

    use crate::bank::Bank;

    #[test]
    fn test_bank_will_create_account_successfully() {
        let customer: Customer = Customer::builder(1).build();
        let repo: InMemoryRepo = InMemoryRepo::new();
        let mut bank = Bank::new(Arc::new(repo));
        let account_id = bank.create_account(customer.id).unwrap();

        assert_eq!(account_id, 1);
        let account = bank.repo.get(account_id).unwrap().unwrap();

        assert_eq!(account.owner, customer.id);
        assert_eq!(account.balance.0, 0.into());
    }

    #[test]
    fn test_bank_will_process_deposit() {
        let repo = Arc::new(InMemoryRepo::default());
        let mut bank = Bank::new(repo);

        let customer = Customer::builder(1).build();
        let account_id = bank.create_account(customer.id).unwrap();

        let result = bank.process(account_id, Transaction::Deposit(Money(100.into())));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 100.into());
    }
}
