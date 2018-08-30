use bigint::{Address, M256, U256};
use std::collections::HashMap;

use errors::{Error, Result};

pub struct Storage {
    storage: HashMap<U256, U256>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    fn insert(&mut self, key: U256, value: U256) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &U256) -> Result<U256> {
        Ok(self.storage.get(key).unwrap_or(&U256::zero()).clone())
    }
}

pub struct AccountState {
    code: Vec<u8>,
    nonce: u32,
    balance: u32,
    storage: Storage,
}

impl AccountState {
    fn new() -> Self {
        AccountState {
            code: Vec::new(),
            nonce: 0,
            balance: 0,
            storage: Storage::new(),
        }
    }
}

pub struct AccountManager {
    accounts: HashMap<Address, AccountState>,
}

impl AccountManager {
    pub fn new() -> Self {
        AccountManager {
            accounts: HashMap::new(),
        }
    }

    fn get_account(&self, address: &Address) -> Result<&AccountState> {
        if !self.accounts.contains_key(address) {
            Err(Error::AccountNotFound)
        } else {
            Ok(&self.accounts[address])
        }
    }

    fn get_account_mut(&mut self, address: &Address) -> Result<&mut AccountState> {
        if !self.accounts.contains_key(address) {
            Err(Error::AccountNotFound)
        } else {
            Ok(self.accounts.get_mut(address).unwrap())
        }
    }

    pub fn balance(&self, address: &Address) -> Result<u32> {
        let account = self.get_account(address)?;
        Ok(account.balance)
    }

    pub fn code(&self, address: &Address) -> Result<Vec<u8>> {
        let account = self.get_account(address)?;
        Ok(account.code.clone())
    }

    pub fn get_storage(&self, address: &Address, offset: &U256) -> Result<U256> {
        let account = self.get_account(address)?;
        Ok(account.storage.get(offset)?)
    }

    pub fn insert_storage(&mut self, address: &Address, offset: U256, value: U256) -> Result<()> {
        let account = self.get_account_mut(address)?;
        account.storage.insert(offset, value);
        Ok(())
    }
}
