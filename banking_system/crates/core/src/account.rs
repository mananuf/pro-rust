use rust_decimal::Decimal;

use crate::{
    customer::CustomerId,
    errors::{DomainError, RepoError},
};

pub type AccountId = u64;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Money(Decimal);

pub trait AccountRepository {
    fn create(&self, account: Account) -> Result<(), RepoError>;
    fn get(&self, id: AccountId) -> Result<Option<Account>, RepoError>;
    fn update(&self, account: Account) -> Result<(), RepoError>;
}

#[derive(Debug, Default)]
pub enum AccountStatus {
    #[default]
    Active,
    Frozen,
    Closed,
}

pub enum Transaction {
    Deposit(Money),
    Withdraw(Money),
    Transfer { to: AccountId, amount: Money },
}

#[derive(Debug, Default)]
pub struct Account {
    pub id: AccountId,
    pub owner: CustomerId,
    pub balance: Money,
    pub status: AccountStatus,
}

impl Account {
    pub fn builder(id: AccountId, owner: CustomerId) -> AccountBuilder {
        AccountBuilder {
            id,
            owner,
            ..Default::default()
        }
    }

    pub fn deposit(&mut self, amount: Money) -> Result<Money, DomainError> {
        if amount.0 <= 0.into() {
            return Err(DomainError::NegativeAmount(
                "amount must be greater than zero".to_string(),
            ));
        }

        if let AccountStatus::Closed = self.status {
            return Err(DomainError::ClosedAccount(
                "cannot deposit money into a closed account".to_string(),
            ));
        }

        self.balance.0 += amount.0;

        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<Money, DomainError> {
        if amount.0 <= 0.into() {
            return Err(DomainError::NegativeAmount(
                "amount must be greater than zero".to_string(),
            ));
        }

        match self.status {
            AccountStatus::Closed => {
                return Err(DomainError::ClosedAccount(
                    "cannot withdraw money from a closed account".to_string(),
                ));
            }
            AccountStatus::Frozen => {
                return Err(DomainError::FrozenAccount(
                    "cannot withdraw money from a frozen account".to_string(),
                ));
            }
            _ => {}
        }

        self.balance.0 -= amount.0;

        Ok(self.balance)
    }

    pub fn apply_transaction(&mut self, txn: Transaction) -> Result<Money, DomainError> {
        match txn {
            Transaction::Deposit(amount) => self.deposit(amount),
            Transaction::Withdraw(amount) => self.withdraw(amount),
            Transaction::Transfer { to, amount } => self.deposit(amount),
        }
    }
}

#[derive(Debug, Default)]
pub struct AccountBuilder {
    pub id: AccountId,
    pub owner: CustomerId,
    pub balance: Option<Money>,
    pub status: Option<AccountStatus>,
}

impl AccountBuilder {
    pub fn balance(mut self, balance: Money) -> Self {
        self.balance = Some(balance);
        self
    }

    pub fn status(mut self, status: AccountStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn build(self) -> Account {
        Account {
            id: self.id,
            owner: self.owner,
            balance: self.balance.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
        }
    }
}

// impl AccountRepository for Account {
//     fn create(&self, account: Account) -> Result<(), RepoError> {
//         Ok(())
//     }
//     fn get(&self, id: AccountId) -> Result<Option<Account>, RepoError> {
//         Ok(None)
//     }
//     fn update(&self, account: Account) -> Result<(), RepoError> {
//         Ok(())
//     }
// }
