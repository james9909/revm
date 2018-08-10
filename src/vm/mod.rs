mod memory;
mod stack;

use asm::instruction::{Instruction, ProgramReader};
use bigint::U256;
use errors::{Error, Result};
use vm::memory::Memory;
use vm::stack::Stack;

const MAX_STACK_SIZE: usize = 1024;

/// The possible end states of a VM run
enum VMResult {
    SUCCESS,
    FAILURE(Error),
}

/// The result of running an Instruction
enum InstructionResult {
    NOTHING,
    STOP,
    REVERT,
}

/// Information regarding the current state of the VM
struct VMState {
    memory: Memory,
    stack: Stack<U256>,
    gas_available: U256,
    pc: usize,
}

struct VM<'a> {
    reader: ProgramReader<'a>,
    state: VMState,
}

impl<'a> VM<'a> {
    pub fn new(code: &'a [u8], gas_available: U256) -> Self {
        VM {
            reader: ProgramReader::new(code),
            state: VMState {
                memory: Memory::new(),
                stack: Stack::new(MAX_STACK_SIZE),
                gas_available: gas_available,
                pc: 0,
            },
        }
    }

    pub fn step(&mut self) -> Result<InstructionResult> {
        let instruction = self.reader.next_instruction()?;
        match instruction {
            Instruction::PUSH(value) => {
                self.state.stack.push(value);
            }
            Instruction::STOP => return Ok(InstructionResult::STOP),
            Instruction::ADD => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_add(right).0);
            }
            Instruction::MUL => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_mul(right).0);
            }
            Instruction::SUB => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_sub(right).0);
            }
            Instruction::DIV => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                if right == U256::zero() {
                    self.state.stack.push(right);
                } else {
                    self.state.stack.push(left / right);
                }
            }
            Instruction::SDIV => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                if right == U256::zero() {
                    self.state.stack.push(right);
                } else {
                    let (left, sign_left) = to_signed(left);
                    let (right, sign_right) = to_signed(right);
                    let min = (U256::one() << 255) - U256::one();
                }
            }
            _ => return Ok(InstructionResult::STOP),
        };
        Ok(InstructionResult::NOTHING)
    }

    fn run(&mut self) -> VMResult {
        while self.state.pc < self.reader.size() {
            let result = self.step();
            match result {
                Ok(instruction_result) => match instruction_result {
                    InstructionResult::STOP | InstructionResult::NOTHING => {}
                    InstructionResult::REVERT => {
                        return VMResult::FAILURE(Error::Revert);
                    }
                },
                Err(e) => return VMResult::FAILURE(e),
            }
        }
        VMResult::SUCCESS
    }
}

fn to_signed(value: U256) -> (U256, bool) {
    let sign = value.bit(255);
    let signed: U256 = if sign {
        (!value).overflowing_add(U256::one()).0
    } else {
        value
    };
    (value, sign)
}
