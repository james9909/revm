use bigint::U256;
use std::cmp;

use asm::opcode::*;
use errors::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,

    // SHA3
    SHA3,

    // Environmental information
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,

    // Stack, Memory, Storage and Flow Operations
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,

    // Push Operations
    PUSH(U256),

    // Duplication Operations
    DUP(usize),

    // Exchange Operations
    SWAP(usize),

    // Logging Operations
    LOG(usize),

    // System Operations
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

pub struct ProgramReader {
    code: Vec<u8>,
    pub position: usize,
}

impl ProgramReader {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code, position: 0 }
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let start = self.position;
        if start >= self.code.len() {
            return Ok(vec![0; size]);
        }

        let end = cmp::min(start + size, self.code.len());
        let mut result = self.code[start..end].to_vec();
        self.position = end;
        result.resize(size, 0);
        Ok(result)
    }

    pub fn is_done(&self) -> bool {
        self.position >= self.code.len()
    }

    pub fn size(&self) -> usize {
        self.code.len()
    }

    pub fn jump(&mut self, position: usize) {
        self.position = position;
    }

    pub fn next_instruction(&mut self) -> Result<Instruction> {
        if self.position >= self.code.len() {
            return Err(Error::PcOverflow);
        }

        let opcode = self.code[self.position];
        self.position += 1;
        let instruction = match opcode {
            STOP => Instruction::STOP,
            ADD => Instruction::ADD,
            MUL => Instruction::MUL,
            SUB => Instruction::SUB,
            DIV => Instruction::DIV,
            SDIV => Instruction::SDIV,
            MOD => Instruction::MOD,
            SMOD => Instruction::SMOD,
            ADDMOD => Instruction::ADDMOD,
            MULMOD => Instruction::MULMOD,
            EXP => Instruction::EXP,
            SIGNEXTEND => Instruction::SIGNEXTEND,
            LT => Instruction::LT,
            GT => Instruction::GT,
            SLT => Instruction::SLT,
            SGT => Instruction::SGT,
            EQ => Instruction::EQ,
            ISZERO => Instruction::ISZERO,
            AND => Instruction::AND,
            OR => Instruction::OR,
            XOR => Instruction::XOR,
            NOT => Instruction::NOT,
            BYTE => Instruction::BYTE,

            SHA3 => Instruction::SHA3,
            ADDRESS => Instruction::ADDRESS,
            BALANCE => Instruction::BALANCE,
            ORIGIN => Instruction::ORIGIN,
            CALLER => Instruction::CALLER,
            CALLVALUE => Instruction::CALLVALUE,
            CALLDATALOAD => Instruction::CALLDATALOAD,
            CALLDATASIZE => Instruction::CALLDATASIZE,
            CALLDATACOPY => Instruction::CALLDATACOPY,
            CODESIZE => Instruction::CODESIZE,
            CODECOPY => Instruction::CODECOPY,
            GASPRICE => Instruction::GASPRICE,
            EXTCODESIZE => Instruction::EXTCODESIZE,
            EXTCODECOPY => Instruction::EXTCODECOPY,
            RETURNDATASIZE => Instruction::RETURNDATASIZE,
            RETURNDATACOPY => Instruction::RETURNDATACOPY,
            BLOCKHASH => Instruction::BLOCKHASH,
            COINBASE => Instruction::COINBASE,
            TIMESTAMP => Instruction::TIMESTAMP,
            NUMBER => Instruction::NUMBER,
            DIFFICULTY => Instruction::DIFFICULTY,
            GASLIMIT => Instruction::GASLIMIT,

            POP => Instruction::POP,
            MLOAD => Instruction::MLOAD,
            MSTORE => Instruction::MSTORE,
            MSTORE8 => Instruction::MSTORE8,
            SLOAD => Instruction::SLOAD,
            SSTORE => Instruction::SSTORE,
            JUMP => Instruction::JUMP,
            JUMPI => Instruction::JUMPI,
            PC => Instruction::PC,
            MSIZE => Instruction::MSIZE,
            GAS => Instruction::GAS,
            JUMPDEST => Instruction::JUMPDEST,

            PUSH1...PUSH32 => {
                let size = usize::from(opcode - PUSH1 + 1);
                // position is automatically incremented when reading bytes
                let res = self.read_bytes(size);
                match res {
                    Ok(bytes) => Instruction::PUSH(U256::from(bytes.as_slice())),
                    Err(_) => return Err(Error::MissingOperand),
                }
            }
            DUP1...DUP16 => {
                let size = (opcode - DUP1 + 1) as usize;
                Instruction::DUP(size)
            }
            SWAP1...SWAP16 => {
                let size = (opcode - SWAP1 + 1) as usize;
                Instruction::SWAP(size)
            }
            LOG0...LOG4 => {
                let size = (opcode - LOG0) as usize;
                Instruction::LOG(size)
            }

            CREATE => Instruction::CREATE,
            CALL => Instruction::CALL,
            CALLCODE => Instruction::CALLCODE,
            RETURN => Instruction::RETURN,
            DELEGATECALL => Instruction::DELEGATECALL,
            STATICCALL => Instruction::STATICCALL,
            REVERT => Instruction::REVERT,
            INVALID => return Err(Error::InvalidOpcode(opcode)),
            SELFDESTRUCT => Instruction::SELFDESTRUCT,
            _ => return Err(Error::InvalidOpcode(opcode)),
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use asm::instruction::{Instruction, ProgramReader};
    use asm::opcode::*;
    use bigint::U256;
    use errors::Error;

    #[test]
    fn test_program_reader() {
        let mut reader = ProgramReader::new(vec![PUSH1, 0x1]);
        assert_eq!(
            reader.next_instruction().unwrap(),
            Instruction::PUSH(U256::from(0x1))
        );
        assert_eq!(reader.next_instruction(), Err(Error::PcOverflow));
    }

    #[test]
    fn test_program_reader_no_operand() {
        for push in PUSH1..PUSH32 {
            let code = vec![push];
            let mut reader = ProgramReader::new(code);
            assert!(reader.next_instruction().is_ok());
        }
    }
}
