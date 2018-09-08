/// A specialized `Result` type for `revm`
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "invalid opcode 0x{:x}", _0)]
    InvalidOpcode(u8),

    #[fail(display = "missing operand")]
    MissingOperand,
    #[fail(display = "operand too large")]
    OperandTooLarge,
    #[fail(display = "StackOverflow")]
    StackOverflow,
    #[fail(display = "StackUnderflow")]
    StackUnderflow,
    #[fail(display = "stack size is too small")]
    StackTooSmall,
    #[fail(display = "requested memory is out of range")]
    MemoryOutOfRange,
    #[fail(display = "requested data is out of range")]
    DataOutOfRange,

    #[fail(display = "PC Overflow")]
    PcOverflow,

    #[fail(display = "Revert")]
    Revert,

    #[fail(display = "account does not exist")]
    AccountNotFound,

    #[fail(display = "out of gas")]
    OutOfGas,

    #[fail(display = "not enough funds")]
    NotEnoughFunds,
}
