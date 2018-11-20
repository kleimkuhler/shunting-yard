/// A generic stack that wraps a `Vec<T>`.
///
/// These methods wrap existing methods on `Vec<T>`. The only reason this
/// struct exists was for my own benefit of implementing.
pub struct Stack<T>(Vec<T>);

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack(Vec::default())
    }
}

impl<T> Stack<T> {
    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Peek the value at the top of the stack.
    ///
    /// # Panics
    ///
    /// The caller is responsible for ensuring the precondition that the stack
    /// is not empty.
    pub fn peek(&self) -> &T {
        let len = self.0.len();
        if len == 0 {
            panic!("cannot peek into an empty stack")
        }

        &self.0[len - 1]
    }

    /// Pop a value off the top of the stack.
    ///
    /// # Panics
    ///
    /// The caller is responsible for ensuring the precondition that the stack
    /// is not empty.
    pub fn pop(&mut self) -> T {
        self.0.pop().expect("cannot pop from an empty stack")
    }

    /// Push a value onto the top of the stack.
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_stack() {
        let stack: Stack<i32> = Stack::default();
        assert!(stack.is_empty())
    }

    #[test]
    #[should_panic(expected = "cannot peek into an empty stack")]
    fn peek_new_stack() {
        let stack: Stack<i32> = Stack::default();
        stack.peek();
    }

    #[test]
    fn test_stack_operations() {
        let mut stack: Stack<i32> = Stack::default();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), &3);
        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.peek(), &1);
        assert_eq!(stack.pop(), 1);
        assert!(stack.is_empty())
    }
}
