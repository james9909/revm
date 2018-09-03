use bigint::Gas;

use asm::instruction::Instruction;
use errors::{Error, Result};

pub enum GasCost {
    ZERO,
    BASE,
    VERYLOW,
    LOW,
    MID,
    HIGH,
    EXTCODE,
    SPECIAL,
}

impl GasCost {
    pub fn get_cost(&self) -> Gas {
        let cost: u64 = match self {
            GasCost::ZERO => 0,
            GasCost::BASE => 2,
            GasCost::VERYLOW => 3,
            GasCost::LOW => 5,
            GasCost::MID => 8,
            GasCost::HIGH => 10,
            GasCost::EXTCODE => 700,
            GasCost::SPECIAL => 0,
        };
        Gas::from(cost)
    }
}

pub struct GasMeter {
    gas: Gas,
}

impl GasMeter {
    pub fn new(gas: Gas) -> Self {
        GasMeter { gas: gas }
    }

    pub fn get_gas(&self) -> Gas {
        self.gas
    }

    pub fn consume(&mut self, amount: Gas) -> Result<()> {
        if self.gas < amount {
            Err(Error::OutOfGas)
        } else {
            self.gas = self.gas - amount;
            Ok(())
        }
    }
}

pub fn get_gas_tier(instruction: &Instruction) -> GasCost {
    match instruction {
        Instruction::STOP | Instruction::RETURN | Instruction::REVERT => GasCost::ZERO,
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
        | Instruction::GAS => GasCost::BASE,
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
        | Instruction::SWAP(_) => GasCost::VERYLOW,
        Instruction::MUL
        | Instruction::DIV
        | Instruction::SDIV
        | Instruction::MOD
        | Instruction::SMOD
        | Instruction::SIGNEXTEND => GasCost::LOW,
        Instruction::ADDMOD | Instruction::MULMOD | Instruction::JUMP => GasCost::MID,
        Instruction::JUMPI => GasCost::HIGH,
        Instruction::EXTCODESIZE => GasCost::EXTCODE,
        _ => GasCost::SPECIAL,
    }
}
