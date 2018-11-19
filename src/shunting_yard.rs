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
    fn associatity(&self) -> Assoc {
        Assoc::Left
    }
}

/// Run the [shunting yard algorithm] on a mathematical expression in infix
/// notation to produce a result in [Reverse Polish Notation].
///
/// [shunting yard algorithm]: https://en.wikipedia.org/wiki/Shunting-yard_algorithm
/// [Reverse Polish Notation]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
trait ShuntingYard<T, I>
where
    T: TokenProperties,
    I: Iterator<Item = T>,
{
    fn produce_postfix(&self, mut tokens: I) {
        let mut parsed: Queue<T> = Queue::default();
        let mut operators: Stack<T> = Stack::default();

        while let Some(token) = tokens.next() {
            // If the precedence is 0 then we have an operand. Operands are
            // always enqueued.
            if token.precedence() == 0 {
                parsed.enqueue(token);
                break;
            }

            // While the operator stack is not empty, check if the current
            // operator should be pushed on to the top.
            while !operators.is_empty() {
                // When compared to `token` -- if the operator's stack
                // precedence is greater, or it is equal and left associative,
                // then remove it from the stack and enqueue it.
                if self.should_stack(&operators, token.precedence()) {
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
    }

    // Helper -- determine is a operator should be stacked
    fn should_stack(&self, operators: &Stack<T>, cur_prec: i32) -> bool {
        let top_op = operators.peek();
        return top_op.precedence() > cur_prec
            || top_op.precedence() == cur_prec && top_op.associatity() == Assoc::Left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Value {
        Int(i64),
    }

    enum Token {
        Value(Value),

        Divide,
        Minus,
        Multiply,
        Plus,
    }

    impl TokenProperties for Token {
        fn precedence(&self) -> i32 {
            match self {
                Token::Minus => 2,
                Token::Multiply => 1,
                Token::Plus => 2,

                _ => 0,
            }
        }
    }

    struct Expression<T, I>
    where
        T: TokenProperties,
        I: Iterator<Item = T>,
    {
        expression: I,
    }

    impl<T, I> Expression<T, I>
    where
        T: TokenProperties,
        I: Iterator<Item = T>,
    {
        fn new(expression: I) -> Self {
            Self { expression }
        }
    }

    #[test]
    fn test_shunting_yard() {
        let mut tokens = &[
            Token::Value(Value::Int(1)),
            Token::Plus,
            Token::Value(Value::Int(2)),
            Token::Multiply,
            Token::Value(Value::Int(3)),
            Token::Minus,
            Token::Value(Value::Int(4)),
        ]
            .iter();

        // let expression = Expression::new(tokens);
    }
}
