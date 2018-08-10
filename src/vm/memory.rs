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

    pub fn write(&mut self, offset: usize, value: U256) -> Result<()> {
        if offset + WORD_SIZE >= self.bytes.len() {
            self.expand(offset + WORD_SIZE);
        }
        for i in 0..WORD_SIZE {
            self.bytes[offset + i] = value.index(i);
        }
        Ok(())
    }

    pub fn read(&mut self, offset: usize) -> Result<U256> {
        if offset + WORD_SIZE >= self.bytes.len() {
            self.expand(offset + WORD_SIZE);
        }
        Ok(self.bytes[offset..offset + WORD_SIZE].as_ref().into())
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
        assert!(memory.write(0, value).is_ok());
        assert_eq!(memory.read(0).unwrap(), value);
    }

    #[test]
    fn read_out_of_range() {
        let mut memory = Memory::new();
        assert_eq!(memory.read(0).unwrap(), U256::from(0));

        let value = U256::from(0xabcd);
        assert!(memory.write(0, value).is_ok());
        assert_eq!(memory.read(1).unwrap(), U256::from(0xabcd00));
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
