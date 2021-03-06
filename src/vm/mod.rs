mod account;
pub mod block;
mod gas;
mod log;
mod memory;
mod stack;

use tiny_keccak::keccak256;

use asm::instruction::{Instruction, ProgramReader};
use bigint::{Address, Gas, U256, U512};
use errors::{Error, Result};
use rlp;
use std::cmp;
use vm::account::AccountManager;
use vm::block::Block;
use vm::gas::GasMeter;
use vm::log::Log;
use vm::memory::Memory;
use vm::stack::Stack;

const MAX_STACK_SIZE: usize = 1024;

/// The possible end states of a VM run
pub enum VMResult {
    SUCCESS,
    FAILURE(Error),
}

/// The result of running an Instruction
pub enum InstructionResult {
    NOTHING,
    STOP,
    REVERT,
}

/// Information regarding the current state of the VM
pub struct VMState {
    pub account_manager: AccountManager,
    pub gas_meter: GasMeter,
    memory: Memory,
    stack: Stack<U256>,
    pub logs: Vec<Log>,

    pub caller: Address,
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub gas_price: Gas,
    pub owner: Address,
    pub origin: Address,
    pub value: U256,
    pub out: Vec<u8>,
}

pub struct VM {
    reader: ProgramReader,
    pub state: VMState,
    pub block: Block,
}

impl VM {
    pub fn new(code: Vec<u8>, block: Block, gas_price: Gas, gas_available: Gas) -> Self {
        VM {
            reader: ProgramReader::new(code),
            state: VMState {
                account_manager: AccountManager::new(),
                gas_meter: GasMeter::new(gas_available),
                memory: Memory::new(),
                stack: Stack::new(MAX_STACK_SIZE),
                logs: Vec::new(),

                code: Vec::new(),
                caller: Address::from(0),
                data: Vec::new(),
                gas_price,
                owner: Address::from(0),
                origin: Address::from(0),
                value: U256::zero(),
                out: Vec::new(),
            },
            block,
        }
    }

    pub fn step(&mut self) -> Result<InstructionResult> {
        if self.reader.is_done() {
            return Ok(InstructionResult::STOP);
        }
        let instruction = self.reader.next_instruction()?;
        let gas_cost = self.state.gas_meter.gas_cost(&self, &instruction)?;
        let mem_cost = self
            .state
            .gas_meter
            .memory_cost(&self.state.stack, &instruction)?;
        let gas_refund = self.state.gas_meter.gas_refund(&self, &instruction)?;

        self.state.gas_meter.consume(gas_cost)?;
        match instruction {
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
                    let a = U512::from(a);
                    let b = U512::from(b);
                    let c = U512::from(c);
                    U256::from(a.overflowing_add(b).0.overflowing_rem(c).0)
                })?;
            }
            Instruction::MULMOD => {
                let a = self.state.stack.pop()?;
                let b = self.state.stack.pop()?;
                let c = self.state.stack.pop()?;
                self.state.stack.push(if c == U256::zero() {
                    c
                } else {
                    let a = U512::from(a);
                    let b = U512::from(b);
                    let c = U512::from(c);
                    U256::from(a.overflowing_mul(b).0.overflowing_rem(c).0)
                })?;
            }
            Instruction::EXP => {
                let base = self.state.stack.pop()?;
                let power = self.state.stack.pop()?;

                self.state.stack.push(base.overflowing_pow(power).0)?;
            }
            Instruction::SIGNEXTEND => {
                let position = self.state.stack.pop()?;
                let number = self.state.stack.pop()?;
                self.state.stack.push(if position > U256::from(32) {
                    number
                } else {
                    let bit_position = (position.low_u64() * 8 + 7) as usize;
                    let bit = number.bit(bit_position);

                    let mask = (U256::one() << bit_position) - U256::one();
                    if bit {
                        number | !mask
                    } else {
                        number & mask
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
                let bytes = self.state.memory.read(offset, amount)?;

                let result = keccak256(&bytes);
                self.state.stack.push(U256::from(&result[..]))?;
            }
            Instruction::ADDRESS => {
                self.state.stack.push(address_to_u256(self.state.owner))?;
            }
            Instruction::BALANCE => {
                let address = u256_to_address(self.state.stack.pop()?);
                let balance = self.state.account_manager.balance(&address)?;
                self.state.stack.push(balance)?;
            }
            Instruction::ORIGIN => {
                self.state.stack.push(address_to_u256(self.state.origin))?;
            }
            Instruction::CALLER => {
                self.state.stack.push(address_to_u256(self.state.caller))?;
            }
            Instruction::CALLVALUE => {
                self.state.stack.push(self.state.value)?;
            }
            Instruction::CALLDATALOAD => {
                let offset = self.state.stack.pop()?;
                let start = if offset > usize::max_value().into() {
                    self.state.data.len()
                } else {
                    offset.low_u64() as usize
                };
                let end = cmp::min(start + 32, self.state.data.len());
                let data_slice = &self.state.data[start..end];

                let mut value = Vec::new();
                for i in 0..32 {
                    value.push(if i < data_slice.len() {
                        data_slice[i]
                    } else {
                        0
                    });
                }

                self.state.stack.push(U256::from(value.as_slice()))?;
            }
            Instruction::CALLDATASIZE => {
                self.state.stack.push(U256::from(self.state.data.len()))?;
            }
            Instruction::CALLDATACOPY => {
                let mem_offset = self.state.stack.pop()?;
                let data_offset = self.state.stack.pop()?.low_u64() as usize;
                let data_size = self.state.stack.pop()?.low_u64() as usize;

                if data_offset > self.state.data.len() {
                    return Ok(InstructionResult::NOTHING);
                }
                if data_offset.overflowing_add(data_size).1 {
                    return Ok(InstructionResult::NOTHING);
                }

                let end = cmp::min(data_offset + data_size, self.state.data.len());
                let data_slice = &self.state.data[data_offset..end];

                let mut value = Vec::new();
                for i in 0..32 {
                    value.push(if i < data_slice.len() {
                        data_slice[i]
                    } else {
                        0
                    });
                }
                self.state
                    .memory
                    .write(mem_offset, U256::from(value.as_slice()))?;
            }
            Instruction::CODESIZE => {
                self.state.stack.push(U256::from(self.state.code.len()))?;
            }
            Instruction::CODECOPY => {
                let mem_offset = self.state.stack.pop()?;
                let code_offset = self.state.stack.pop()?;
                let code_size = self.state.stack.pop()?;

                let code_offset = code_offset.low_u64() as usize;
                let code_size = code_size.low_u64() as usize;
                if code_offset < self.state.data.len() {
                    let code_slice = &self.state.code[code_offset..code_offset + code_size];
                    let mut value = Vec::new();
                    for i in 0..32 {
                        value.push(if i < code_slice.len() {
                            code_slice[i]
                        } else {
                            0
                        });
                    }
                    self.state
                        .memory
                        .write(mem_offset, U256::from(value.as_slice()))?;
                }
            }
            Instruction::GASPRICE => {
                self.state.stack.push(self.state.gas_price.into())?;
            }
            Instruction::EXTCODESIZE => {
                let address = u256_to_address(self.state.stack.pop()?);
                let code = self.state.account_manager.code(&address)?;
                self.state.stack.push(U256::from(code.len()))?;
            }
            Instruction::EXTCODECOPY => {
                let address = u256_to_address(self.state.stack.pop()?);
                let code = self.state.account_manager.code(&address)?;

                let mem_offset = self.state.stack.pop()?;
                let code_offset = self.state.stack.pop()?.low_u64() as usize;
                let code_size = self.state.stack.pop()?.low_u64() as usize;
                let value = &code[code_offset..code_offset + code_size];
                self.state.memory.write(mem_offset, U256::from(value))?;
            }
            Instruction::RETURNDATASIZE => {
                // TODO
            }
            Instruction::RETURNDATACOPY => {
                // TODO
            }
            Instruction::BLOCKHASH => {
                // Implement the VMTest version of BLOCKHASH
                // hash of block number n is sha3(n)
                let number = self.state.stack.pop()?;
                let mut bytes = vec![0; 32];
                number.to_big_endian(&mut bytes);

                let result = keccak256(&bytes[..]);
                self.state.stack.push(U256::from(&result[..]))?;
            }
            Instruction::COINBASE => {
                self.state
                    .stack
                    .push(address_to_u256(self.block.beneficiary))?;
            }
            Instruction::TIMESTAMP => {
                self.state.stack.push(self.block.timestamp)?;
            }
            Instruction::NUMBER => {
                self.state.stack.push(self.block.number)?;
            }
            Instruction::DIFFICULTY => {
                self.state.stack.push(self.block.difficulty)?;
            }
            Instruction::GASLIMIT => {
                self.state.stack.push(self.block.gas_limit)?;
            }
            Instruction::POP => {
                self.state.stack.pop()?;
            }
            Instruction::MLOAD => {
                let offset = self.state.stack.pop()?;
                let value = self.state.memory.read(offset, U256::from(32))?;
                self.state.stack.push(U256::from(value.as_slice()))?;
            }
            Instruction::MSTORE => {
                let offset = self.state.stack.pop()?;
                let value = self.state.stack.pop()?;
                self.state.memory.write(offset, value)?;
            }
            Instruction::MSTORE8 => {
                let offset = self.state.stack.pop()?;
                let value = self.state.stack.pop()?;
                self.state.memory.write_byte(offset, value.byte(0))?;
            }
            Instruction::SLOAD => {
                let offset = self.state.stack.pop()?;
                let data = self
                    .state
                    .account_manager
                    .get_storage(&self.state.owner, &offset)?;
                self.state.stack.push(data)?;
            }
            Instruction::SSTORE => {
                let offset = self.state.stack.pop()?;
                let value = self.state.stack.pop()?;
                self.state
                    .account_manager
                    .insert_storage(&self.state.owner, offset, value)?;
            }
            Instruction::JUMP => {
                let offset = self.state.stack.pop()?.low_u64() as usize;
                self.reader.jump(offset);
            }
            Instruction::JUMPI => {
                let offset = self.state.stack.pop()?.low_u64() as usize;
                let cond = self.state.stack.pop()?;
                if cond != U256::zero() {
                    self.reader.jump(offset);
                }
            }
            Instruction::PC => {
                self.state
                    .stack
                    .push(U256::from(self.reader.position - 1))?;
            }
            Instruction::MSIZE => {
                self.state
                    .stack
                    .push((self.state.gas_meter.memory_cost * Gas::from(32u64)).into())?;
            }
            Instruction::GAS => {
                self.state
                    .stack
                    .push(self.state.gas_meter.remaining_gas().into())?;
            }
            Instruction::JUMPDEST => {}
            Instruction::PUSH(value) => {
                self.state.stack.push(value)?;
            }
            Instruction::DUP(position) => {
                self.state.stack.dup(position)?;
            }
            Instruction::SWAP(position) => {
                self.state.stack.swap(position)?;
            }
            Instruction::LOG(length) => {
                let offset = self.state.stack.pop()?;
                let amount = self.state.stack.pop()?;
                let data = self.state.memory.read(offset, amount)?;

                let mut topics = Vec::new();
                for _ in 0..length {
                    topics.push(self.state.stack.pop()?.into());
                }
                self.state.logs.push(Log {
                    address: self.state.owner,
                    data: U256::from(data.as_slice()),
                    topics: topics,
                })
            }
            Instruction::RETURN => {
                let offset = self.state.stack.pop()?;
                let amount = self.state.stack.pop()?;
                self.state.out = self.state.memory.read(offset, amount)?;
            }
            _ => return Ok(InstructionResult::STOP),
        };
        self.state.gas_meter.refunded_gas = self.state.gas_meter.refunded_gas + gas_refund;
        Ok(InstructionResult::NOTHING)
    }

    pub fn run(&mut self) -> VMResult {
        loop {
            let result = self.step();
            match result {
                Ok(instruction_result) => match instruction_result {
                    InstructionResult::STOP => {
                        break;
                    }
                    InstructionResult::NOTHING => {}
                    InstructionResult::REVERT => {
                        return VMResult::FAILURE(Error::Revert);
                    }
                },
                Err(e) => return VMResult::FAILURE(e),
            }
        }
        VMResult::SUCCESS
    }

    pub fn log_hash(&self) -> U256 {
        let encoded = rlp::encode_list(&self.state.logs[..]);
        let result = keccak256(&encoded[..]);
        U256::from(&result[..])
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

fn u256_to_address(value: U256) -> Address {
    let mut bytes = vec![0; 32];
    value.to_big_endian(&mut bytes);
    Address::from(&bytes[0..20])
}
