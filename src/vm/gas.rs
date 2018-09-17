use bigint::{Gas, U256};

use asm::instruction::Instruction;
use errors::{Error, Result};
use vm::stack::Stack;
use vm::VM;

const G_ZERO: usize = 0;
const G_BASE: usize = 2;
const G_VERYLOW: usize = 3;
const G_LOW: usize = 5;
const G_MID: usize = 8;
const G_HIGH: usize = 10;
const G_EXTCODE: usize = 700;
const G_BALANCE: usize = 400;
const G_SLOAD: usize = 50;
const G_JUMPDEST: usize = 1;
const G_SSET: usize = 20000;
const G_SRESET: usize = 5000;
const R_SCLEAR: usize = 15000;
const R_SELFDESTRUCT: usize = 24000;
const G_SELFDESTRUCT: usize = 5000;
const G_CREATE: usize = 32000;
const G_CODEDEPOSIT: usize = 200;
const G_CALL: usize = 700;
const G_CALLVALUE: usize = 9000;
const G_CALLSTIPEND: usize = 2300;
const G_NEWACCOUNT: usize = 25000;
const G_EXP: usize = 10;
const G_EXPBYTE: usize = 10;
const G_MEMORY: usize = 3;
const G_LOG: usize = 375;
const G_LOGDATA: usize = 8;
const G_LOGTOPIC: usize = 375;
const G_SHA3: usize = 30;
const G_SHA3WORD: usize = 6;
const G_COPY: usize = 3;
const G_BLOCKHASH: usize = 20;
const G_QUADDIVISOR: usize = 100;

pub struct GasMeter {
    pub gas_limit: Gas,
    pub gas_cost: Gas,
    pub memory_cost: Gas,
    pub refunded_gas: Gas,
}

impl GasMeter {
    pub fn new(gas: Gas) -> Self {
        GasMeter {
            gas_limit: gas,
            gas_cost: Gas::zero(),
            memory_cost: Gas::zero(),
            refunded_gas: Gas::zero(),
        }
    }

    pub fn remaining_gas(&self) -> Gas {
        self.gas_limit - self.gas_cost - memory_gas_cost(self.memory_cost) + self.refunded_gas
    }

    pub fn consume(&mut self, amount: Gas) -> Result<()> {
        if self.remaining_gas() < amount {
            Err(Error::OutOfGas)
        } else {
            self.gas_cost = self.gas_cost + amount;
            Ok(())
        }
    }

    pub fn gas_refund(&self, vm: &VM, instruction: &Instruction) -> Result<Gas> {
        let refund: Gas = match instruction {
            Instruction::SSTORE => {
                let offset = vm.state.stack.peek(0)?;
                let value = vm.state.stack.peek(1)?;
                let address = vm.state.owner;

                let result = vm.state.account_manager.get_storage(&address, &offset);
                if value.is_zero() && (result.is_ok() && !result.unwrap().is_zero()) {
                    R_SCLEAR.into()
                } else {
                    Gas::zero()
                }
            }
            _ => Gas::zero(),
        };
        Ok(refund)
    }

    pub fn gas_cost(&self, vm: &VM, instruction: &Instruction) -> Result<Gas> {
        let cost: Gas = match instruction {
            Instruction::STOP | Instruction::RETURN | Instruction::REVERT => G_ZERO.into(),
            Instruction::ADDRESS
            | Instruction::ORIGIN
            | Instruction::CALLER
            | Instruction::CALLVALUE
            | Instruction::CALLDATASIZE
            | Instruction::CODESIZE
            | Instruction::GASPRICE
            | Instruction::COINBASE
            | Instruction::TIMESTAMP
            | Instruction::NUMBER
            | Instruction::DIFFICULTY
            | Instruction::GASLIMIT
            | Instruction::RETURNDATASIZE
            | Instruction::POP
            | Instruction::PC
            | Instruction::MSIZE
            | Instruction::GAS => G_BASE.into(),
            Instruction::ADD
            | Instruction::SUB
            | Instruction::NOT
            | Instruction::LT
            | Instruction::GT
            | Instruction::SLT
            | Instruction::SGT
            | Instruction::EQ
            | Instruction::ISZERO
            | Instruction::AND
            | Instruction::OR
            | Instruction::XOR
            | Instruction::BYTE
            | Instruction::CALLDATALOAD
            | Instruction::MLOAD
            | Instruction::MSTORE
            | Instruction::MSTORE8
            | Instruction::PUSH(_)
            | Instruction::DUP(_)
            | Instruction::SWAP(_) => G_VERYLOW.into(),
            Instruction::MUL
            | Instruction::DIV
            | Instruction::SDIV
            | Instruction::MOD
            | Instruction::SMOD
            | Instruction::SIGNEXTEND => G_LOW.into(),
            Instruction::ADDMOD | Instruction::MULMOD | Instruction::JUMP => G_MID.into(),
            Instruction::JUMPI => G_HIGH.into(),
            Instruction::EXTCODESIZE => G_EXTCODE.into(),
            Instruction::BALANCE => G_BALANCE.into(),
            Instruction::BLOCKHASH => G_BLOCKHASH.into(),
            Instruction::CREATE => G_CREATE.into(),
            Instruction::JUMPDEST => G_JUMPDEST.into(),
            Instruction::SLOAD => G_SLOAD.into(),

            Instruction::SHA3 => {
                let amount = vm.state.stack.peek(1)?;
                let words = (amount.bits() + 31) >> 5;
                Gas::from(G_SHA3 + (G_SHA3WORD * words))
            }
            Instruction::LOG(v) => {
                let amount = vm.state.stack.peek(1)?;
                Gas::from(G_LOG)
                    + Gas::from(G_LOGDATA) * Gas::from(amount)
                    + Gas::from(G_LOGTOPIC) * Gas::from(*v)
            }
            Instruction::EXTCODECOPY => {
                let amount = vm.state.stack.peek(3)?;
                let words = (amount.bits() + 31) >> 5;
                Gas::from(G_EXTCODE) + Gas::from(G_COPY) * Gas::from(words)
            }
            Instruction::CODECOPY | Instruction::CALLDATACOPY | Instruction::RETURNDATACOPY => {
                let amount = vm.state.stack.peek(2)?;
                let words = (Gas::from(amount) + Gas::from(31u64)) / Gas::from(32u64);
                Gas::from(G_VERYLOW) + Gas::from(G_COPY) * words
            }
            Instruction::EXP => {
                let power = vm.state.stack.peek(1)?;
                if power.is_zero() {
                    Gas::from(G_EXP)
                } else {
                    Gas::from(G_EXP) + Gas::from(G_EXPBYTE) * Gas::from(1 + power.log2floor() / 8)
                }
            }
            Instruction::SSTORE => {
                let offset = vm.state.stack.peek(0)?;
                let value = vm.state.stack.peek(1)?;
                let address = vm.state.owner;

                let result = vm.state.account_manager.get_storage(&address, &offset);
                if !value.is_zero() && (result.is_err() || result.unwrap().is_zero()) {
                    G_SSET.into()
                } else {
                    G_SRESET.into()
                }
            }
            Instruction::CALL => {
                // TODO
                G_CALL.into()
            }
            Instruction::CALLCODE => {
                // TODO
                G_CALL.into()
            }
            Instruction::DELEGATECALL => {
                // TODO
                G_CALL.into()
            }
            Instruction::STATICCALL => {
                // TODO
                G_CALL.into()
            }
            Instruction::SELFDESTRUCT => {
                // TODO
                G_SELFDESTRUCT.into()
            }
            Instruction::INVALID => 0u64.into(),
        };
        Ok(cost)
    }

    pub fn memory_cost(&mut self, stack: &Stack<U256>, instruction: &Instruction) -> Result<Gas> {
        let cost = match instruction {
            Instruction::MSTORE | Instruction::MLOAD => {
                let offset = stack.peek(0)?;
                self.memory_expand(offset, U256::from(32u64))
            }
            Instruction::MSTORE8 => {
                let offset = stack.peek(0)?;
                self.memory_expand(offset, U256::one())
            }
            Instruction::SHA3 | Instruction::RETURN | Instruction::REVERT => {
                let offset: U256 = stack.peek(0)?;
                let amount: U256 = stack.peek(1)?;
                self.memory_expand(offset, amount)
            }
            Instruction::CODECOPY | Instruction::CALLDATACOPY | Instruction::RETURNDATACOPY => {
                let offset: U256 = stack.peek(0)?;
                let amount: U256 = stack.peek(2)?;
                self.memory_expand(offset, amount)
            }
            Instruction::EXTCODECOPY => {
                let offset: U256 = stack.peek(0)?;
                let amount: U256 = stack.peek(3)?;
                self.memory_expand(offset, amount)
            }
            Instruction::CREATE => {
                let offset: U256 = stack.peek(1)?;
                let amount: U256 = stack.peek(2)?;
                self.memory_expand(offset, amount)
            }
            Instruction::CALL | Instruction::CALLCODE => {
                let in_offset: U256 = stack.peek(3)?;
                let in_amount: U256 = stack.peek(4)?;
                let out_offset: U256 = stack.peek(5)?;
                let out_amount: U256 = stack.peek(6)?;
                self.memory_expand(in_offset, in_amount)
                    .max(self.memory_expand(out_offset, out_amount))
            }
            _ => self.memory_cost,
        };
        let check = memory_gas_cost(cost);
        if check > self.gas_limit - self.gas_cost {
            Err(Error::OutOfGas)
        } else {
            self.memory_cost = cost;
            Ok(check)
        }
    }

    fn memory_expand(&self, offset: U256, amount: U256) -> Gas {
        if amount.is_zero() {
            return self.memory_cost;
        }
        let new_size: Gas =
            (Gas::from(offset) + Gas::from(amount) + Gas::from(31u64)) / Gas::from(32u64);
        self.memory_cost.max(new_size)
    }
}

fn memory_gas_cost(cost: Gas) -> Gas {
    Gas::from(G_MEMORY) * cost + cost * cost / Gas::from(512u64)
}
