use queue::Queue;
use stack::Stack;

#[derive(PartialEq)]
enum Assoc {
    Left,
    None,
    Right,
}

/// Expression tokens must implement this trait.
trait TokenProperties {
    // Return the precedence of the current token
    fn precedence(&self) -> i32;

    // Return the associativity of the current token
    fn associativity(&self) -> Assoc {
        Assoc::Left
    }
}

struct ShuntingYard<T, I>
where
    T: TokenProperties,
    I: IntoIterator<Item = T>,
{
    input: I,
}

impl<T, I> ShuntingYard<T, I>
where
    T: TokenProperties,
    I: IntoIterator<Item = T>,
{
    fn new(input: I) -> Self {
        Self { input }
    }

    /// Run the [shunting yard algorithm] on a mathematical expression in
    /// infix notation to produce a result in RPN.
    ///
    /// [shunting yard algorithm]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    fn produce_postfix(self) -> Queue<T> {
        let mut tokens = self.input.into_iter();
        let mut parsed: Queue<T> = Queue::default();
        let mut operators: Stack<T> = Stack::default();

        while let Some(token) = tokens.next() {
            // If the precedence is 0 then we have an operand. Operands are
            // always enqueued.
            if token.precedence() == 0 {
                parsed.enqueue(token);
                continue;
            }

            // While the operator stack is not empty, check if the current
            // operator should be pushed on to the top.
            //
            // When compared to `token` -- if the operator's stack
            // precedence is greater, or it is equal and left associative,
            // then remove it from the stack and enqueue it.
            while !operators.is_empty() {
                if Self::should_stack(&operators.peek(), token.precedence()) {
                    parsed.enqueue(operators.pop())
                } else {
                    break;
                }
            }

            operators.push(token);
        }

        // Enqueue the remaining operators
        while !operators.is_empty() {
            parsed.enqueue(operators.pop())
        }

        parsed
    }

    fn should_stack(cur_top: &T, cur_prec: i32) -> bool {
        cur_top.precedence() > cur_prec
            || cur_top.precedence() == cur_prec && cur_top.associativity() == Assoc::Left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Value {
        Int(i64),
    }

    #[derive(Debug, PartialEq)]
    enum Token {
        Value(Value),

        Minus,
        Multiply,
        Plus,
    }

    impl TokenProperties for Token {
        fn precedence(&self) -> i32 {
            match &self {
                Token::Minus => 1,
                Token::Multiply => 2,
                Token::Plus => 1,

                _ => 0,
            }
        }
    }

    #[test]
    fn test_shunting_yard() {
        let input = vec![
            Token::Value(Value::Int(1)),
            Token::Plus,
            Token::Value(Value::Int(2)),
            Token::Multiply,
            Token::Value(Value::Int(3)),
            Token::Minus,
            Token::Value(Value::Int(4)),
        ];
        let output = vec![
            Token::Value(Value::Int(1)),
            Token::Value(Value::Int(2)),
            Token::Value(Value::Int(3)),
            Token::Multiply,
            Token::Plus,
            Token::Value(Value::Int(4)),
            Token::Minus,
        ];
        let shunting_yard = ShuntingYard::new(input);

        assert_eq!(shunting_yard.produce_postfix(), Queue::new(output))
    }
}
