use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidExpression(String),
    ConsecutiveOperators,
    DivisionByZero,
    TooManyOperators,
    EmptyExpression,
}

impl Display for Error {
    // Overriding the Display trait to provide a user-friendly error message
    // This trait is used to format the error messages when they are printed.
    // The `fmt` method takes a mutable reference to a `Formatter` and returns a `Result`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
            Error::InvalidOperator(msg) => write!(f, "Invalid operator: {}", msg),
            Error::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            Error::DivisionByZero => write!(f, "Division by zero"),
            Error::ConsecutiveOperators => write!(f, "Consecutive operators are not allowed"),
            Error::TooManyOperators => write!(f, "Too many operators in the expression"),
            Error::EmptyExpression => write!(f, "The expression cannot be empty"),
        }
    }
}

pub fn parse_expression(expression: Vec<&str>) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    for token in expression {
        if let Ok(num) = token.parse::<f64>() {
            tokens.push(Token::Number(num));
        } else if token.len() == 1 && "+-*/".contains(token) {
            tokens.push(Token::Operator(token.chars().next().unwrap()));
        } else {
            return Err(Error::InvalidExpression(token.to_string()));
        }
    }

    // Check if we have at least one number and one operator
    if tokens.is_empty() {
        return Err(Error::EmptyExpression);
    }

    Ok(tokens)
}

fn precedence(op: char) -> i32 {
    // Define operator precedence
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(op: char, b: f64, a: f64) -> Result<f64, Error> {
    match op {
        '+' => {
            Ok(a + b)
        }
        '-' => {
            Ok(a - b)
        }
        '*' => {
            Ok(a * b)
        }
        '/' => {
            if b == 0.0 {
                Err(Error::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        _ => panic!("Unknown operator: {}", op),
    }
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, Error> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for &token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(&top_op) = operators.last() {
                    // If there is an operator on the stack, and it has greater precedence, then pop it to output
                    if precedence(top_op) >= precedence(op) {
                        output.push(Token::Operator(operators.pop().unwrap()));
                    } else {
                        break;
                    }
                }
                // If the operator is not already in the stack, push it
                operators.push(op);
            }
        }
    }

    // Pop all remaining operators from the stack
    while let Some(op) = operators.pop() {
        output.push(Token::Operator(op));
    }

    Ok(output)
}

fn evaluate_rpn(tokens: &[Token]) -> Result<f64, Error> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(*num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(Error::InvalidExpression("Not enough operands for operator".to_string()));
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                match apply_operator(*op, a, b) {
                    Ok(result) => stack.push(result),
                    Err(e) => return Err(e), // Propagate the error if it occurs
                }
            }
        }
    }

    if stack.len() != 1 {
        return Err(Error::TooManyOperators);
    }

    Ok(stack[0])
}

pub fn evaluate_expression(expression: &[Token]) -> Result<f64, Error> {
    let rpn_tokens = to_rpn(expression)?; // Convert the expression to Reverse Polish Notation (RPN). The ? operator propagates errors. Propagating errors means that if an error occurs, it will be returned to the caller instead of panicking.
    match evaluate_rpn(&rpn_tokens) {
        Ok(result) => Ok(result),
        Err(e) => Err(e), // Convert the Error enum to a String for user-friendly output
    }
}

pub fn print_help_doc() {
    println!("Commands:");
    println!(" Enter an expression to evaluate it (e.g., 2 + 2)");
    println!(" Type 'quit' or 'q' to exit the calculator");
    println!(" Type 'help' to see this help message");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rpn() {
        let tokens = parse_expression(vec!["2", "+", "3", "*", "4"]).unwrap();
        let rpn = to_rpn(&tokens).unwrap();
        assert_eq!(
            rpn,
            vec![
                Token::Number(2.0),
                Token::Number(3.0),
                Token::Number(4.0),
                Token::Operator('*'),
                Token::Operator('+')
            ]
        );
    }

    #[test]
    fn test_apply_operator() {
        assert_eq!(apply_operator('+', 2.0, 3.0), Ok(5.0));
        assert_eq!(apply_operator('-', 5.0, 3.0), Ok(-2.0));
        assert_eq!(apply_operator('*', 2.0, 3.0), Ok(6.0));
        assert_eq!(apply_operator('/', 2.0, 6.0), Ok(3.0));
        assert!(matches!(
            apply_operator('/', 0.0, 4.0),
            Err(Error::DivisionByZero)
        ));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence('+'), 1);
        assert_eq!(precedence('-'), 1);
        assert_eq!(precedence('*'), 2);
        assert_eq!(precedence('/'), 2);
        assert_eq!(precedence('%'), 0);
    }
}
