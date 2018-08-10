use bigint::U256;
use errors::Result;

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

    pub fn read(&mut self, offset: U256, amount: U256) -> Result<U256> {
        let offset = offset.low_u64() as usize;
        let amount = amount.low_u64() as usize;
        if offset + amount >= self.bytes.len() {
            self.expand(offset + amount);
        }
        Ok(self.bytes[offset..offset + amount].as_ref().into())
    }
}

#[cfg(test)]
mod tests {
    use bigint::U256;
    use vm::memory::Memory;

    #[test]
    fn can_read_write() {
        let mut memory = Memory::new();
        let value = U256::from(0x12);
        assert!(memory.write(U256::zero(), value).is_ok());
        assert_eq!(memory.read(U256::zero(), U256::from(32)).unwrap(), value);
    }

    #[test]
    fn read_out_of_range() {
        let mut memory = Memory::new();
        assert_eq!(
            memory.read(U256::zero(), U256::from(32)).unwrap(),
            U256::from(0)
        );

        let value = U256::from(0xabcd);
        assert!(memory.write(U256::zero(), value).is_ok());
        assert_eq!(
            memory.read(U256::one(), U256::from(32)).unwrap(),
            U256::from(0xabcd00)
        );
    }

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
