use errors::{Error, Result};

pub struct Stack<T> {
    data: Vec<T>,
    capacity: usize,
}

#[allow(dead_code)]
/// Stack implementation for the EVM
impl<T: Copy> Stack<T> {
    fn new(capacity: usize) -> Stack<T> {
        Stack {
            data: Vec::with_capacity(capacity),
            capacity: capacity,
        }
    }

    fn push(&mut self, item: T) -> Result<()> {
        if self.data.len() >= self.capacity {
            return Err(Error::StackOverflow);
        }
        self.data.push(item);
        Ok(())
    }

    fn peek(&self) -> Result<T> {
        match self.data.len() {
            0 => Err(Error::StackUnderflow),
            _ => Ok(self.data[self.data.len() - 1].clone()),
        }
    }

    fn pop(&mut self) -> Result<T> {
        match self.data.len() {
            0 => Err(Error::StackUnderflow),
            _ => Ok(self.data.pop().unwrap()),
        }
    }

    fn dup(&mut self, position: usize) -> Result<()> {
        match self.data.len() {
            0 => Err(Error::StackOverflow),
            _ => {
                if self.data.len() < position {
                    Err(Error::StackTooSmall)
                } else {
                    let dup = self.data[self.data.len() - position].clone();
                    self.push(dup)
                }
            }
        }
    }

    fn swap(&mut self, position: usize) -> Result<()> {
        match self.data.len() {
            0 => Err(Error::StackOverflow),
            _ => {
                if self.data.len() - 1 < position {
                    Err(Error::StackTooSmall)
                } else {
                    let pos1 = self.data.len() - 1;
                    let pos2 = pos1 - position;
                    self.data.swap(pos1, pos2);
                    Ok(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vm::stack::Stack;

    #[test]
    fn basic_operations() {
        let mut stack = Stack::new(10);

        for i in 1..10 {
            assert!(stack.push(i).is_ok());
        }

        for i in 10..1 {
            assert_eq!(stack.peek().unwrap(), i);
            assert_eq!(stack.pop().unwrap(), i);
        }
    }

    #[test]
    fn underflow_error() {
        let mut stack: Stack<i32> = Stack::new(10);
        assert!(stack.pop().is_err());
        assert!(stack.peek().is_err());

        assert!(stack.dup(1).is_err());
        assert!(stack.swap(1).is_err());
    }

    #[test]
    fn overflow_error() {
        let mut stack = Stack::new(3);
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.push(3).is_ok());

        assert!(stack.push(4).is_err());
        assert!(stack.dup(1).is_err());
    }

    #[test]
    fn dup_works() {
        let mut stack = Stack::new(10);
        for i in 1..5 {
            assert!(stack.push(i).is_ok());
        }

        assert_eq!(stack.data, [1, 2, 3, 4]);
        assert!(stack.dup(1).is_ok());
        assert_eq!(stack.data, [1, 2, 3, 4, 4]);
        assert!(stack.dup(5).is_ok());
        assert_eq!(stack.data, [1, 2, 3, 4, 4, 1]);
    }

    #[test]
    fn swap_works() {
        let mut stack = Stack::new(10);
        for i in 1..5 {
            assert!(stack.push(i).is_ok());
        }

        assert_eq!(stack.data, [1, 2, 3, 4]);
        assert!(stack.swap(1).is_ok());
        assert_eq!(stack.data, [1, 2, 4, 3]);
        assert!(stack.swap(3).is_ok());
        assert_eq!(stack.data, [3, 2, 4, 1]);
    }

    #[test]
    fn out_of_bounds() {
        let mut stack = Stack::new(10);
        for i in 1..5 {
            assert!(stack.push(i).is_ok());
        }

        assert!(stack.swap(5).is_err());
        assert!(stack.dup(5).is_err());
    }
}
