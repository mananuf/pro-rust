use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bank_core::{
    account::{Account, AccountId, AccountRepository},
    errors::RepoError,
};

pub struct InMemoryRepo {
    pub store: Arc<RwLock<HashMap<AccountId, Account>>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl AccountRepository for InMemoryRepo {
    fn create(&self, account: Account) -> Result<(), RepoError> {
        let mut store = self.store.write().map_err(|_| RepoError::LockPoisened)?;
        store.insert(account.id, account);
        Ok(())
    }
    fn get(&self, id: AccountId) -> Result<Option<Account>, RepoError> {
        let store = self.store.read().map_err(|_| RepoError::LockPoisened)?;
        Ok(store.get(&id).cloned())
    }
    fn update(&self, account: Account) -> Result<(), RepoError> {
        let mut store = self.store.write().map_err(|_| RepoError::LockPoisened)?;
        store.insert(account.id, account);
        Ok(())
    }
}

impl Default for InMemoryRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]

pub mod tests {
    use bank_core::{account::{Account, AccountRepository}, customer::Customer};

    use crate::storage::InMemoryRepo;

    #[test]
    fn test_will_create_account_and_store_to_in_memory_repo_successfully() {
        let customer: Customer = Customer::builder(1).build();
        let account: Account = Account::builder(1, customer.id).build();
        let repo: InMemoryRepo = InMemoryRepo::new();
        let _ = repo.create(account.clone());

        let response = repo.get(account.id);
        assert!(response.is_ok());
        assert!(matches!(response, Ok(Some(_))));
        assert_eq!(response.ok().unwrap(), Some(account))
    }

    #[test]
    fn test_will_update_account_in_memory_repo_successfully() {
        let customer: Customer = Customer::builder(1).build();
        let mut account: Account = Account::builder(1, customer.id).build();
        let repo: InMemoryRepo = InMemoryRepo::new();
        let _ = repo.create(account.clone());

        let response = repo.get(account.id);
        assert!(response.is_ok());
        assert!(matches!(response, Ok(Some(_))));
        assert_eq!(response.ok().unwrap(), Some(account.clone()));
        
        let customer: Customer = Customer::builder(2).build();
        account.id = customer.id;
        let _ = repo.update(account.clone());

        let response = repo.get(account.id);
        assert!(response.is_ok());
        assert!(matches!(response, Ok(Some(_))));
        assert_eq!(response.ok().unwrap(), Some(account.clone()));
    }
}