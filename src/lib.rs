#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
}

pub fn parse_expression(expression: Vec<&str>) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for token in expression {
        if let Ok(num) = token.parse::<f64>() {
            tokens.push(Token::Number(num));
        } else if token.len() == 1 && "+-*/".contains(token) {
            tokens.push(Token::Operator(token.chars().next().unwrap()));
        } else {
            return Err(format!("Invalid token: {}", token));
        }
    }

    // Check if we have at least one number and one operator
    if tokens.is_empty() {
        return Err("No valid tokens found in the expression".to_string());
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

fn apply_operator(op: char, b: f64, a: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => b / a, // Note: order is important for division
        _ => panic!("Unknown operator: {}", op),
    }
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, String> {
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

fn evaluate_rpn(tokens: &[Token]) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(*num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Not enough operands for operator".to_string());
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = apply_operator(*op, a, b);
                stack.push(result);
            }
        }
    }

    if stack.len() != 1 {
        return Err("Too many operands left on the stack".to_string());
    }

    Ok(stack[0])
}

pub fn evaluate_expression(expression: &[Token]) -> Result<f64, String> {
    let rpn_tokens = to_rpn(expression)?; // Convert the expression to Reverse Polish Notation (RPN). The ? operator propagates errors. Propagating errors means that if an error occurs, it will be returned to the caller instead of panicking.
    evaluate_rpn(&rpn_tokens)
}

pub fn print_help_doc() {
    println!("Commands:");
    println!(" Enter an expression to evaluate it (e.g., 2 + 2)");
    println!(" Type 'quit' or 'q' to exit the calculator");
    println!(" Type 'help' to see this help message");
}
