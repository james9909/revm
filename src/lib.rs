extern crate bigint;
#[macro_use]
extern crate failure;

pub mod asm;
pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
