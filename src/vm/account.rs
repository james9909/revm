use bigint::{Address, M256, U256};
use std::collections::HashMap;

use errors::Result;

pub struct Storage {
    storage: HashMap<U256, M256>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    fn insert(&mut self, key: U256, value: M256) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &U256) -> Result<M256> {
        Ok(self.storage.get(key).unwrap_or(&M256::zero()).clone())
    }
}

pub struct AccountState {
    nonce: u32,
    balance: u32,
    storage: Storage,
}

impl AccountState {
    fn new() -> Self {
        AccountState {
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

    pub fn has_address(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn balance(&self, address: &Address) -> Result<u32> {
        Ok(self.accounts[address].balance)
    }
}
