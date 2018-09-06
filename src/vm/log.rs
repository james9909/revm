use bigint::{Address, H256, U256};
use rlp::{Encodable, RlpStream};

#[derive(Debug)]
pub struct Log {
    pub address: Address,
    pub data: U256,
    pub topics: Vec<H256>,
}

impl Encodable for Log {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}
