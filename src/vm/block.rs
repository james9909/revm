use bigint::{Address, U256};

pub struct Block {
    pub beneficiary: Address,
    pub difficulty: U256,
    pub gas_limit: U256,
    pub number: U256,
    pub timestamp: U256,
}
