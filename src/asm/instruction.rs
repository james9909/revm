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
    DUP(u8),

    // Exchange Operations
    SWAP(u8),

    // Logging Operations
    LOG(u8),

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

struct ProgramReader<'a> {
    code: &'a [u8],
    position: usize,
}

impl<'a> ProgramReader<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            code: code,
            position: 0,
        }
    }

    fn read_bytes(&mut self, size: usize) -> Result<&[u8]> {
        let start = self.position;
        if start >= self.code.len() {
            return Err(Error::PcOverflow);
        }

        let end = cmp::min(start + size, self.code.len());
        let result = &self.code[start..end];
        self.position = end;
        Ok(result)
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
                    Ok(bytes) => Instruction::PUSH(U256::from(bytes)),
                    Err(_) => return Err(Error::MissingOperand),
                }
            }
            DUP1...DUP16 => {
                let size = opcode - DUP1 + 1;
                self.position += size as usize;
                Instruction::DUP(size)
            }
            SWAP1...SWAP16 => {
                let size = opcode - SWAP1 + 1;
                self.position += size as usize;
                Instruction::SWAP(size)
            }
            LOG1...LOG4 => {
                let size = opcode - LOG1 + 1;
                self.position += size as usize;
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
        let mut reader = ProgramReader::new(&[PUSH1, 0x1]);
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
            let mut reader = ProgramReader::new(&code);
            assert_eq!(reader.next_instruction(), Err(Error::MissingOperand));
        }
    }
}
