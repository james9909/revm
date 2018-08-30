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
    balance: U256,
    storage: Storage,
}

impl AccountState {
    fn new() -> Self {
        AccountState {
            code: Vec::new(),
            nonce: 0,
            balance: U256::zero(),
            storage: Storage::new(),
        }
    }

    pub fn set_balance(&mut self, balance: U256) {
        self.balance = balance;
    }

    pub fn get_balance(&self) -> U256 {
        self.balance
    }

    pub fn set_code(&mut self, code: Vec<u8>) {
        self.code = code;
    }

    pub fn get_code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
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

    pub fn create_account(&mut self, address: &Address, code: Vec<u8>, balance: U256) {
        if self.accounts.contains_key(address) {
            return;
        }
        let mut account_state = AccountState::new();
        account_state.set_balance(balance);
        account_state.set_code(code);
        self.accounts.insert(*address, account_state);
    }

    pub fn get_account(&self, address: &Address) -> Result<&AccountState> {
        if !self.accounts.contains_key(address) {
            Err(Error::AccountNotFound)
        } else {
            Ok(&self.accounts[address])
        }
    }

    pub fn get_account_mut(&mut self, address: &Address) -> Result<&mut AccountState> {
        if !self.accounts.contains_key(address) {
            Err(Error::AccountNotFound)
        } else {
            Ok(self.accounts.get_mut(address).unwrap())
        }
    }

    pub fn balance(&self, address: &Address) -> Result<U256> {
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
