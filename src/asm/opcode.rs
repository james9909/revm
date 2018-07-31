// pub enum Opcode {
//     // Stop and Arithmetic Operations
//     STOP,
//     ADD,
//     MUL,
//     SUB,
//     DIV,
//     SDIV,
//     MOD,
//     SMOD,
//     ADDMOD,
//     MULMOD,
//     EXP,
//     SIGNEXTEND,
//     LT,
//     GT,
//     SLT,
//     SGT,
//     EQ,
//     ISZERO,
//     AND,
//     OR,
//     XOR,
//     NOT,
//     BYTE,

//     // SHA3
//     SHA3,
//     ADDRESS,
//     BALANCE,
//     ORIGIN,
//     CALLER,
//     CALLVALUE,
//     CALLDATALOAD,
//     CALLDATASIZE,
//     CALLDATACOPY,
//     CODESIZE,
//     CODECOPY,
//     GASPRICE,
//     EXTCODESIZE,
//     EXTCODECOPY,
//     RETURNDATASIZE,
//     RETURNDATACOPY,
//     BLOCKHASH,
//     COINBASE,
//     TIMESTAMP,
//     NUMBER,
//     DIFFICULTY,
//     GASLIMIT,

//     // Stack, Memory, Storage and Flow Operations
//     POP,
//     MLOAD,
//     MSTORE,
//     MSTORE8,
//     SLOAD,
//     SSTORE,
//     JUMP,
//     JUMPI,
//     PC,
//     MSIZE,
//     GAS,
//     JUMPDEST,

//     // Push Operations
//     PUSH(u8),

//     // Duplication Operations
//     DUP(u8),

//     // Exchange Operations
//     SWAP(u8),

//     // Logging Operations
//     LOG(u8),

//     // System Operations
//     CREATE,
//     CALL,
//     CALLCODE,
//     RETURN,
//     DELEGATECALL,
//     STATICCALL,
//     REVERT,
//     INVALID,
//     SELFDESTRUCT,
// }

// impl From<u8> for Opcode {
//     fn from(opcode: u8) -> Self {
//         match opcode {
//             0x00 => Opcode::STOP,
//             0x01 => Opcode::ADD,
//             0x02 => Opcode::MUL,
//             0x03 => Opcode::SUB,
//             0x04 => Opcode::DIV,
//             0x05 => Opcode::SDIV,
//             0x06 => Opcode::MOD,
//             0x07 => Opcode::SMOD,
//             0x08 => Opcode::ADDMOD,
//             0x09 => Opcode::MULMOD,
//             0x0a => Opcode::EXP,
//             0x0b => Opcode::SIGNEXTEND,
//             0x10 => Opcode::LT,
//             0x11 => Opcode::GT,
//             0x12 => Opcode::SLT,
//             0x13 => Opcode::SGT,
//             0x14 => Opcode::EQ,
//             0x15 => Opcode::ISZERO,
//             0x16 => Opcode::AND,
//             0x17 => Opcode::OR,
//             0x18 => Opcode::XOR,
//             0x19 => Opcode::NOT,
//             0x1a => Opcode::BYTE,
//             0x20 => Opcode::SHA3,
//             0x30 => Opcode::ADDRESS,
//             0x31 => Opcode::BALANCE,
//             0x32 => Opcode::ORIGIN,
//             0x33 => Opcode::CALLER,
//             0x34 => Opcode::CALLVALUE,
//             0x35 => Opcode::CALLDATALOAD,
//             0x36 => Opcode::CALLDATASIZE,
//             0x37 => Opcode::CALLDATACOPY,
//             0x38 => Opcode::CODESIZE,
//             0x39 => Opcode::CODECOPY,
//             0x3a => Opcode::GASPRICE,
//             0x3b => Opcode::EXTCODESIZE,
//             0x3c => Opcode::EXTCODECOPY,
//             0x3d => Opcode::RETURNDATASIZE,
//             0x3e => Opcode::RETURNDATACOPY,
//             0x40 => Opcode::BLOCKHASH,
//             0x41 => Opcode::COINBASE,
//             0x42 => Opcode::TIMESTAMP,
//             0x43 => Opcode::NUMBER,
//             0x44 => Opcode::DIFFICULTY,
//             0x45 => Opcode::GASLIMIT,
//             0x50 => Opcode::POP,
//             0x51 => Opcode::MLOAD,
//             0x52 => Opcode::MSTORE,
//             0x53 => Opcode::MSTORE8,
//             0x54 => Opcode::SLOAD,
//             0x55 => Opcode::SSTORE,
//             0x56 => Opcode::JUMP,
//             0x57 => Opcode::JUMPI,
//             0x58 => Opcode::PC,
//             0x59 => Opcode::MSIZE,
//             0x5a => Opcode::GAS,
//             0x5b => Opcode::JUMPDEST,
//             0x60...0x7f => Opcode::PUSH(0x60 - opcode + 1),
//             0x80...0x8f => Opcode::DUP(0x80 - opcode + 1),
//             0x90...0x9f => Opcode::SWAP(0x80 - opcode + 1),
//             0xa0...0xa4 => Opcode::LOG(0xa0 - opcode + 1),
//             0xf0 => Opcode::CREATE,
//             0xf1 => Opcode::CALL,
//             0xf2 => Opcode::CALLCODE,
//             0xf3 => Opcode::RETURN,
//             0xf4 => Opcode::DELEGATECALL,
//             0xfa => Opcode::STATICCALL,
//             0xfd => Opcode::REVERT,
//             0xfe => Opcode::INVALID,
//             0xff => Opcode::SELFDESTRUCT,
//             _ => Opcode::INVALID,
//         }
//     }
// }

/// EVM instruction opcodes
pub type Opcode = u8;

pub const STOP: Opcode = 0x00;
pub const ADD: Opcode = 0x01;
pub const MUL: Opcode = 0x02;
pub const SUB: Opcode = 0x03;
pub const DIV: Opcode = 0x04;
pub const SDIV: Opcode = 0x05;
pub const MOD: Opcode = 0x06;
pub const SMOD: Opcode = 0x07;
pub const ADDMOD: Opcode = 0x08;
pub const MULMOD: Opcode = 0x09;
pub const EXP: Opcode = 0x0a;
pub const SIGNEXTEND: Opcode = 0x0b;
pub const LT: Opcode = 0x10;
pub const GT: Opcode = 0x11;
pub const SLT: Opcode = 0x12;
pub const SGT: Opcode = 0x13;
pub const EQ: Opcode = 0x14;
pub const ISZERO: Opcode = 0x15;
pub const AND: Opcode = 0x16;
pub const OR: Opcode = 0x17;
pub const XOR: Opcode = 0x18;
pub const NOT: Opcode = 0x19;
pub const BYTE: Opcode = 0x1a;
pub const SHA3: Opcode = 0x20;
pub const ADDRESS: Opcode = 0x30;
pub const BALANCE: Opcode = 0x31;
pub const ORIGIN: Opcode = 0x32;
pub const CALLER: Opcode = 0x33;
pub const CALLVALUE: Opcode = 0x34;
pub const CALLDATALOAD: Opcode = 0x35;
pub const CALLDATASIZE: Opcode = 0x36;
pub const CALLDATACOPY: Opcode = 0x37;
pub const CODESIZE: Opcode = 0x38;
pub const CODECOPY: Opcode = 0x39;
pub const GASPRICE: Opcode = 0x3a;
pub const EXTCODESIZE: Opcode = 0x3b;
pub const EXTCODECOPY: Opcode = 0x3c;
pub const RETURNDATASIZE: Opcode = 0x3d;
pub const RETURNDATACOPY: Opcode = 0x3e;
pub const BLOCKHASH: Opcode = 0x40;
pub const COINBASE: Opcode = 0x41;
pub const TIMESTAMP: Opcode = 0x42;
pub const NUMBER: Opcode = 0x43;
pub const DIFFICULTY: Opcode = 0x44;
pub const GASLIMIT: Opcode = 0x45;
pub const POP: Opcode = 0x50;
pub const MLOAD: Opcode = 0x51;
pub const MSTORE: Opcode = 0x52;
pub const MSTORE8: Opcode = 0x53;
pub const SLOAD: Opcode = 0x54;
pub const SSTORE: Opcode = 0x55;
pub const JUMP: Opcode = 0x56;
pub const JUMPI: Opcode = 0x57;
pub const PC: Opcode = 0x58;
pub const MSIZE: Opcode = 0x59;
pub const GAS: Opcode = 0x5a;
pub const JUMPDEST: Opcode = 0x5b;
pub const PUSH1: Opcode = 0x60;
pub const PUSH2: Opcode = 0x61;
pub const PUSH3: Opcode = 0x62;
pub const PUSH4: Opcode = 0x63;
pub const PUSH5: Opcode = 0x64;
pub const PUSH6: Opcode = 0x65;
pub const PUSH7: Opcode = 0x66;
pub const PUSH8: Opcode = 0x67;
pub const PUSH9: Opcode = 0x68;
pub const PUSH10: Opcode = 0x69;
pub const PUSH11: Opcode = 0x6a;
pub const PUSH12: Opcode = 0x6b;
pub const PUSH13: Opcode = 0x6c;
pub const PUSH14: Opcode = 0x6d;
pub const PUSH15: Opcode = 0x6e;
pub const PUSH16: Opcode = 0x6f;
pub const PUSH17: Opcode = 0x70;
pub const PUSH18: Opcode = 0x71;
pub const PUSH19: Opcode = 0x72;
pub const PUSH20: Opcode = 0x73;
pub const PUSH21: Opcode = 0x74;
pub const PUSH22: Opcode = 0x75;
pub const PUSH23: Opcode = 0x76;
pub const PUSH24: Opcode = 0x77;
pub const PUSH25: Opcode = 0x78;
pub const PUSH26: Opcode = 0x79;
pub const PUSH27: Opcode = 0x7a;
pub const PUSH28: Opcode = 0x7b;
pub const PUSH29: Opcode = 0x7c;
pub const PUSH30: Opcode = 0x7d;
pub const PUSH31: Opcode = 0x7e;
pub const PUSH32: Opcode = 0x7f;
pub const DUP1: Opcode = 0x80;
pub const DUP2: Opcode = 0x81;
pub const DUP3: Opcode = 0x82;
pub const DUP4: Opcode = 0x83;
pub const DUP5: Opcode = 0x84;
pub const DUP6: Opcode = 0x85;
pub const DUP7: Opcode = 0x86;
pub const DUP8: Opcode = 0x87;
pub const DUP9: Opcode = 0x88;
pub const DUP10: Opcode = 0x89;
pub const DUP11: Opcode = 0x8a;
pub const DUP12: Opcode = 0x8b;
pub const DUP13: Opcode = 0x8c;
pub const DUP14: Opcode = 0x8d;
pub const DUP15: Opcode = 0x8e;
pub const DUP16: Opcode = 0x8f;
pub const SWAP1: Opcode = 0x90;
pub const SWAP2: Opcode = 0x91;
pub const SWAP3: Opcode = 0x92;
pub const SWAP4: Opcode = 0x93;
pub const SWAP5: Opcode = 0x94;
pub const SWAP6: Opcode = 0x95;
pub const SWAP7: Opcode = 0x96;
pub const SWAP8: Opcode = 0x97;
pub const SWAP9: Opcode = 0x98;
pub const SWAP10: Opcode = 0x99;
pub const SWAP11: Opcode = 0x9a;
pub const SWAP12: Opcode = 0x9b;
pub const SWAP13: Opcode = 0x9c;
pub const SWAP14: Opcode = 0x9d;
pub const SWAP15: Opcode = 0x9e;
pub const SWAP16: Opcode = 0x9f;
pub const LOG0: Opcode = 0xa0;
pub const LOG1: Opcode = 0xa1;
pub const LOG2: Opcode = 0xa2;
pub const LOG3: Opcode = 0xa3;
pub const LOG4: Opcode = 0xa4;
pub const CREATE: Opcode = 0xf0;
pub const CALL: Opcode = 0xf1;
pub const CALLCODE: Opcode = 0xf2;
pub const RETURN: Opcode = 0xf3;
pub const DELEGATECALL: Opcode = 0xf4;
pub const STATICCALL: Opcode = 0xfa;
pub const REVERT: Opcode = 0xfd;
pub const INVALID: Opcode = 0xfe;
pub const SELFDESTRUCT: Opcode = 0xff;
