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

#[derive(Debug, Default, Clone, PartialEq)]
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

#[derive(Debug, Default, Clone)]
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

        if amount.0 > self.balance.0 {
            return Err(DomainError::InsufficientFunds(
                "insufficient balance".into(),
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
            Transaction::Transfer { .. } => Err(DomainError::Unsupported("use Bank::transfer instead".into())),
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

#[cfg(test)]
pub mod tests {
    use crate::{
        account::{Account, AccountStatus, Money},
        customer::Customer,
        errors::DomainError,
    };

    #[test]
    fn test_account_will_be_created_successfully() {
        let customer = Customer::builder(1).build();
        let account = Account::builder(1, customer.id).build();

        assert_eq!(account.id, 1);
        assert_eq!(account.balance.0, 0.into());
        assert_eq!(account.status, AccountStatus::Active);
        assert_eq!(account.owner, customer.id);
    }

    #[test]
    fn test_account_will_allow_deposit_for_active_accounts() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Active);

        let deposit = account.deposit(Money(100.into()));

        assert!(deposit.is_ok());
        assert_eq!(deposit.ok().unwrap(), Money(200.into()));
    }

    #[test]
    fn test_account_will_not_allow_deposit_0_amount() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Active);

        let deposit = account.deposit(Money(0.into()));

        assert!(matches!(deposit, Err(DomainError::NegativeAmount(_))));
        assert!(deposit.is_err());
        assert_eq!(
            deposit.err().unwrap(),
            DomainError::NegativeAmount("amount must be greater than zero".to_string())
        );
    }

    #[test]
    fn test_will_not_allow_deposit_to_a_closed_account() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .status(AccountStatus::Closed)
            .build();

        assert_eq!(account.status, AccountStatus::Closed);

        let deposit = account.deposit(Money(10.into()));

        assert!(matches!(deposit, Err(DomainError::ClosedAccount(_))));
        assert!(deposit.is_err());
        assert_eq!(
            deposit.err().unwrap(),
            DomainError::ClosedAccount("cannot deposit money into a closed account".to_string())
        );
    }

    #[test]
    fn test_will_allow_deposit_to_a_frozen_account() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .status(AccountStatus::Frozen)
            .balance(Money(5.into()))
            .build();

        assert_eq!(account.status, AccountStatus::Frozen);
        assert_eq!(account.balance.0, 5.into());

        let deposit = account.deposit(Money(10.into()));

        assert!(matches!(deposit, Ok(Money(_))));
        assert!(deposit.is_ok());
        assert_eq!(deposit.ok().unwrap(), Money(15.into()));
    }

    #[test]
    fn test_will_allow_withdrawal_for_active_accounts() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Active);

        let withdraw = account.withdraw(Money(100.into()));

        assert!(withdraw.is_ok());
        assert_eq!(withdraw.ok().unwrap(), Money(0.into()));
    }

    #[test]
    fn test_will_not_allow_withdrawal_for_active_accounts_when_amount_is_greater_than_balance() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Active);

        let withdraw = account.withdraw(Money(101.into()));

        assert!(withdraw.is_err());
        assert_eq!(
            withdraw.err().unwrap(),
            DomainError::InsufficientFunds("insufficient balance".into())
        );
    }

    #[test]
    fn test_will_not_allow_withdrawal_for_frozen_account() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .status(AccountStatus::Frozen)
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Frozen);

        let withdraw = account.withdraw(Money(10.into()));

        assert!(withdraw.is_err());
        assert_eq!(
            withdraw.err().unwrap(),
            DomainError::FrozenAccount("cannot withdraw money from a frozen account".to_string())
        );
    }

        #[test]
    fn test_will_not_allow_withdrawal_for_closed_account() {
        let customer = Customer::builder(1).build();
        let mut account = Account::builder(1, customer.id)
            .balance(Money(100.into()))
            .status(AccountStatus::Closed)
            .build();

        assert_eq!(account.balance.0, 100.into());
        assert_eq!(account.status, AccountStatus::Closed);

        let withdraw = account.withdraw(Money(10.into()));

        assert!(withdraw.is_err());
        assert_eq!(
            withdraw.err().unwrap(),
            DomainError::ClosedAccount("cannot withdraw money from a closed account".to_string())
        );
    }
}
