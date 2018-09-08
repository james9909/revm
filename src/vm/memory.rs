use bigint::U256;
use errors::{Error, Result};

/// Size of a word in bytes
const WORD_SIZE: usize = 32;

pub struct Memory {
    bytes: Vec<u8>,
}

#[allow(dead_code)]
/// Memory model used for the EVM
impl Memory {
    pub fn new() -> Memory {
        Memory { bytes: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    pub fn expand(&mut self, new_size: usize) {
        if new_size > self.bytes.len() {
            self.bytes.resize(new_size, 0);
        }
    }

    pub fn write(&mut self, offset: U256, value: U256) -> Result<()> {
        let offset = offset.low_u64() as usize;
        if offset + WORD_SIZE >= self.bytes.len() {
            self.expand(offset + WORD_SIZE);
        }
        for i in 0..WORD_SIZE {
            self.bytes[offset + i] = value.index(i);
        }
        Ok(())
    }

    pub fn write_byte(&mut self, offset: U256, value: u8) -> Result<()> {
        let offset = offset.low_u64() as usize;
        if offset >= self.bytes.len() {
            self.expand(offset + 1);
        }
        self.bytes[offset] = value;
        Ok(())
    }

    pub fn read(&self, offset: U256, amount: U256) -> Result<Vec<u8>> {
        if offset.overflowing_add(amount).1 {
            return Err(Error::MemoryOutOfRange);
        }

        let mut copy: Vec<u8> = Vec::new();
        let mut i = offset;
        while i < offset + amount {
            copy.push(self.read_byte(i.into()));
            i = i + U256::one();
        }
        Ok(copy)
    }

    pub fn read_byte(&self, offset: U256) -> u8 {
        if offset > U256::from(usize::max_value()) {
            return 0;
        }

        let offset = offset.as_usize();
        if offset >= self.bytes.len() {
            return 0;
        }

        self.bytes[offset]
    }
}

#[cfg(test)]
mod tests {
    use bigint::U256;
    use vm::memory::Memory;

    #[test]
    fn can_expand() {
        let mut memory = Memory::new();
        memory.expand(32);
        assert_eq!(memory.size(), 32);

        // Memory size shouldn't change unless we expand past its current capacity
        memory.expand(31);
        assert_eq!(memory.size(), 32);

        memory.expand(64);
        assert_eq!(memory.size(), 64);
    }
}
