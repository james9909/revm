extern crate bigint;
#[macro_use]
extern crate failure;
extern crate tiny_keccak;

pub mod asm;
pub mod errors;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
