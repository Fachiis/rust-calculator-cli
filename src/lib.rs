use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
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
    MismatchedBracket,
    EmptyBrackets,
    NotFinite,
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
            Error::MismatchedBracket => write!(f, "Mismatched brackets"),
            Error::EmptyBrackets => write!(f, "Empty brackets"),
            Error::NotFinite => write!(f, "Result is not finite"),
        }
    }
}

pub fn parse_expression(expression: Vec<&str>) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    for token in expression {
        if let Ok(num) = token.parse::<f64>() {
            tokens.push(Token::Number(num));
        } else if token == "(" {
            tokens.push(Token::LeftParen);
        } else if token == ")" {
            tokens.push(Token::RightParen);
        } else if token.len() == 1 && "+-*/^".contains(token) {
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
        '^' => 3,
        _ => 0,
    }
}

fn is_right_associative(op: char) -> bool {
    op == '^'
}

fn apply_operator(op: char, b: f64, a: f64) -> Result<f64, Error> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err(Error::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        '^' => Ok(a.powf(b)),
        _ => Err(Error::InvalidOperator(op.to_string())),
    }
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, Error> {
    let mut output = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    let mut previous_token: Option<Token> = None;

    for &token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::LeftParen => operators.push(Token::LeftParen),
            Token::RightParen => {
                if matches!(previous_token, Some(Token::LeftParen)) {
                    return Err(Error::EmptyBrackets);
                }

                let mut found_left = false;
                while let Some(top) = operators.pop() {
                    match top {
                        Token::Operator(op) => output.push(Token::Operator(op)),
                        Token::LeftParen => {
                            found_left = true;
                            break;
                        }
                        _ => return Err(Error::MismatchedBracket),
                    }
                }

                if !found_left {
                    return Err(Error::MismatchedBracket);
                }
            }
            Token::Operator(op) => {
                while let Some(top) = operators.last().copied() {
                    match top {
                        Token::Operator(top_op) => {
                            let should_pop = if is_right_associative(op) {
                                precedence(top_op) > precedence(op)
                            } else {
                                precedence(top_op) >= precedence(op)
                            };

                            if should_pop {
                                operators.pop();
                                output.push(Token::Operator(top_op));
                            } else {
                                break;
                            }
                        }
                        Token::LeftParen => break,
                        _ => {
                            return Err(Error::InvalidExpression(
                                "Invalid token on stack".to_string(),
                            ))
                        }
                    }
                }

                operators.push(Token::Operator(op));
            }
        }

        previous_token = Some(token);
    }

    // Pop all remaining operators from the stack
    while let Some(top) = operators.pop() {
        match top {
            Token::Operator(op) => output.push(Token::Operator(op)),
            Token::LeftParen | Token::RightParen => return Err(Error::MismatchedBracket),
            _ => {
                return Err(Error::InvalidExpression(
                    "Invalid token on stack".to_string(),
                ))
            }
        }
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
                    return Err(Error::InvalidExpression(
                        "Not enough operands for operator".to_string(),
                    ));
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = apply_operator(*op, right, left)?;
                if !result.is_finite() {
                    return Err(Error::NotFinite);
                }
                stack.push(result);
            }
            Token::LeftParen | Token::RightParen => {
                return Err(Error::InvalidExpression(
                    "Parenthesis found in RPN".to_string(),
                ));
            }
        }
    }

    if stack.len() != 1 {
        return Err(Error::TooManyOperators);
    }

    Ok(stack[0])
}

pub fn evaluate_expression(expression: &[Token]) -> Result<f64, Error> {
    let rpn_tokens = to_rpn(expression)?;
    evaluate_rpn(&rpn_tokens)
}

pub fn print_help_doc() {
    println!("Commands:");
    println!(" Enter an expression to evaluate it (e.g., 2 + 2)");
    println!(" Parentheses are supported (e.g., ( 2 + 3 ) * 4)");
    println!(" Exponentiation is supported with ^ (e.g., 2 ^ 3 ^ 2)");
    println!(" Type 'quit' or 'q' to exit the calculator");
    println!(" Type 'help' to see this help message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_with_parentheses() {
        let tokens = parse_expression(vec!["(", "2", "+", "3", ")", "*", "4"]).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Number(2.0),
                Token::Operator('+'),
                Token::Number(3.0),
                Token::RightParen,
                Token::Operator('*'),
                Token::Number(4.0),
            ]
        );
    }

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
    fn test_to_rpn_with_parentheses() {
        let tokens = parse_expression(vec!["(", "2", "+", "3", ")", "*", "4"]).unwrap();
        let rpn = to_rpn(&tokens).unwrap();
        assert_eq!(
            rpn,
            vec![
                Token::Number(2.0),
                Token::Number(3.0),
                Token::Operator('+'),
                Token::Number(4.0),
                Token::Operator('*'),
            ]
        );
    }

    #[test]
    fn test_to_rpn_right_associative_exponent() {
        let tokens = parse_expression(vec!["2", "^", "3", "^", "2"]).unwrap();
        let rpn = to_rpn(&tokens).unwrap();
        assert_eq!(
            rpn,
            vec![
                Token::Number(2.0),
                Token::Number(3.0),
                Token::Number(2.0),
                Token::Operator('^'),
                Token::Operator('^'),
            ]
        );
    }

    #[test]
    fn test_apply_operator() {
        assert_eq!(apply_operator('+', 2.0, 3.0), Ok(5.0));
        assert_eq!(apply_operator('-', 5.0, 3.0), Ok(-2.0));
        assert_eq!(apply_operator('*', 2.0, 3.0), Ok(6.0));
        assert_eq!(apply_operator('/', 2.0, 6.0), Ok(3.0));
        assert_eq!(apply_operator('^', 3.0, 2.0), Ok(8.0));
        assert!(matches!(
            apply_operator('/', 0.0, 4.0),
            Err(Error::DivisionByZero)
        ));
        assert!(matches!(
            apply_operator('%', 2.0, 1.0),
            Err(Error::InvalidOperator(_))
        ));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence('+'), 1);
        assert_eq!(precedence('-'), 1);
        assert_eq!(precedence('*'), 2);
        assert_eq!(precedence('/'), 2);
        assert_eq!(precedence('^'), 3);
        assert_eq!(precedence('%'), 0);
    }

    #[test]
    fn test_is_right_associative() {
        assert!(is_right_associative('^'));
        assert!(!is_right_associative('*'));
    }

    #[test]
    fn test_bracket_errors() {
        let tokens = parse_expression(vec!["(", "2", "+", "3"]).unwrap();
        assert!(matches!(
            evaluate_expression(&tokens),
            Err(Error::MismatchedBracket)
        ));

        let tokens = parse_expression(vec!["2", "+", "3", ")"]).unwrap();
        assert!(matches!(
            evaluate_expression(&tokens),
            Err(Error::MismatchedBracket)
        ));

        let tokens = parse_expression(vec!["(", ")"]).unwrap();
        assert!(matches!(
            evaluate_expression(&tokens),
            Err(Error::EmptyBrackets)
        ));
    }
}
