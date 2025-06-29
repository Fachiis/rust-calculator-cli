use calculator_cli::{evaluate_expression, parse_expression, Error, Token};

#[test]
fn test_parse_expression() {
    let input = vec!["3", "+", "5"];
    let expected = vec![Token::Number(3.0), Token::Operator('+'), Token::Number(5.0)];
    assert_eq!(parse_expression(input).unwrap(), expected);
}

#[test]
fn test_invalid_expression() {
    let input = vec!["3", "+", "five"];
    let result = parse_expression(input);
    assert!(result.is_err());
}

#[test]
fn test_empty_expression() {
    let input: Vec<&str> = vec![];
    let result = parse_expression(input);
    assert!(result.is_err());
}

#[test]
fn test_single_number() {
    let input = vec!["42"];
    let expected = vec![Token::Number(42.0)];
    assert_eq!(parse_expression(input).unwrap(), expected);
}

#[test]
fn test_evaluate_expression() {
    let input = vec!["3", "+", "5"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 8.0);

    let input = vec!["10", "/", "2"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 5.0);

    let input = vec!["2", "*", "3", "+", "4"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 10.0);
}

#[test]
fn test_division_by_zero() {
    let input = vec!["10", "/", "0"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens);
    assert!(matches!(result, Err(Error::DivisionByZero)));
}