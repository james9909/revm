mod account;
mod memory;
mod stack;

use tiny_keccak::keccak256;

use asm::instruction::{Instruction, ProgramReader};
use bigint::{Address, U256};
use errors::{Error, Result};
use vm::account::AccountState;
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
    owner: Address,
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
                owner: Address::from(0),
            },
        }
    }

    pub fn step(&mut self) -> Result<InstructionResult> {
        let instruction = self.reader.next_instruction()?;
        match instruction {
            Instruction::PUSH(value) => {
                self.state.stack.push(value)?;
            }
            Instruction::STOP => return Ok(InstructionResult::STOP),
            Instruction::ADD => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_add(right).0)?;
            }
            Instruction::MUL => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_mul(right).0)?;
            }
            Instruction::SUB => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left.overflowing_sub(right).0)?;
            }
            Instruction::DIV => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                if right == U256::zero() {
                    self.state.stack.push(right)?;
                } else {
                    self.state.stack.push(left / right)?;
                }
            }
            Instruction::SDIV => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                let result = if right == U256::zero() {
                    right
                } else {
                    let (left, left_sign) = to_signed(left);
                    let (right, right_sign) = to_signed(right);
                    let min = (U256::one() << 255) - U256::one();

                    if left == min && right == !U256::one() {
                        min
                    } else {
                        let sign = left_sign ^ right_sign;
                        set_sign(left.overflowing_div(right).0, sign)
                    }
                };
                self.state.stack.push(result)?;
            }
            Instruction::MOD => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(if right == U256::zero() {
                    right
                } else {
                    left.overflowing_rem(right).0
                })?
            }
            Instruction::SMOD => {
                let (left, left_sign) = to_signed(self.state.stack.pop()?);
                let right = to_signed(self.state.stack.pop()?).0;
                self.state.stack.push(if right == U256::zero() {
                    right
                } else {
                    set_sign(left.overflowing_rem(right).0, left_sign)
                })?;
            }
            Instruction::ADDMOD => {
                let a = self.state.stack.pop()?;
                let b = self.state.stack.pop()?;
                let c = self.state.stack.pop()?;
                self.state.stack.push(if c == U256::zero() {
                    c
                } else {
                    (a.overflowing_add(b).0).overflowing_rem(c).0
                })?;
            }
            Instruction::MULMOD => {
                let a = self.state.stack.pop()?;
                let b = self.state.stack.pop()?;
                let c = self.state.stack.pop()?;
                self.state.stack.push(if c == U256::zero() {
                    c
                } else {
                    (a.overflowing_mul(b).0).overflowing_rem(c).0
                })?;
            }
            Instruction::EXP => {
                let base = self.state.stack.pop()?;
                let power = self.state.stack.pop()?;
                self.state.stack.push(base.overflowing_pow(power).0)?;
            }
            Instruction::SIGNEXTEND => {
                let value = self.state.stack.pop()?;
                self.state.stack.push(if value > U256::from(32) {
                    value
                } else {
                    let bit = (value.low_u64() * 8 + 7) as usize;
                    let mask = (U256::one() << bit) - U256::one();
                    if value.bit(bit) {
                        value | !mask
                    } else {
                        value & mask
                    }
                })?;
            }
            Instruction::LT => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(bool_to_u256(left < right))?;
            }
            Instruction::GT => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(bool_to_u256(left > right))?;
            }
            Instruction::SLT => {
                let (left, left_sign) = to_signed(self.state.stack.pop()?);
                let (right, right_sign) = to_signed(self.state.stack.pop()?);
                let result = match (left_sign, right_sign) {
                    (false, false) => left < right,
                    (true, true) => left > right,
                    (true, false) => true,
                    (false, true) => false,
                };
                self.state.stack.push(bool_to_u256(result))?;
            }
            Instruction::SGT => {
                let (left, left_sign) = to_signed(self.state.stack.pop()?);
                let (right, right_sign) = to_signed(self.state.stack.pop()?);
                let result = match (left_sign, right_sign) {
                    (false, false) => left > right,
                    (true, true) => left < right,
                    (true, false) => false,
                    (false, true) => true,
                };
                self.state.stack.push(bool_to_u256(result))?;
            }
            Instruction::EQ => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(bool_to_u256(left == right))?;
            }
            Instruction::ISZERO => {
                let value = self.state.stack.pop()?;
                self.state.stack.push(bool_to_u256(value == U256::zero()))?;
            }
            Instruction::AND => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left & right)?;
            }
            Instruction::OR => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left | right)?;
            }
            Instruction::XOR => {
                let left = self.state.stack.pop()?;
                let right = self.state.stack.pop()?;
                self.state.stack.push(left ^ right)?;
            }
            Instruction::NOT => {
                let value = self.state.stack.pop()?;
                self.state.stack.push(!value)?;
            }
            Instruction::BYTE => {
                let byte = self.state.stack.pop()?;
                let word = self.state.stack.pop()?;
                self.state.stack.push(if byte > U256::from(32) {
                    U256::zero()
                } else {
                    U256::from(word.byte(byte.as_u64() as usize))
                })?;
            }
            Instruction::SHA3 => {
                let offset = self.state.stack.pop()?;
                let amount = self.state.stack.pop()?;
                let value = self.state.memory.read(offset, amount)?;
                let mut bytes = vec![0; 32];
                value.to_big_endian(&mut bytes);
                let result = keccak256(bytes.as_slice());

                self.state.stack.push(U256::from(&result[..]))?;
            }
            Instruction::ADDRESS => {
                self.state.stack.push(address_to_u256(self.state.owner))?;
            }
            Instruction::BALANCE => {}
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
    let signed = set_sign(value, sign);
    (signed, sign)
}

fn set_sign(value: U256, sign: bool) -> U256 {
    if sign {
        (!value).overflowing_add(U256::one()).0
    } else {
        value
    }
}

fn bool_to_u256(b: bool) -> U256 {
    if b {
        U256::one()
    } else {
        U256::zero()
    }
}

fn address_to_u256(address: Address) -> U256 {
    U256::from(&address[0..20])
}
