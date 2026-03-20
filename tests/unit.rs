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

#[test]
fn test_bracket_evaluation() {
    let input = vec!["(", "2", "+", "3", ")", "*", "4"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 20.0);

    let input = vec!["10", "/", "(", "2", "+", "3", ")"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 2.0);

    let input = vec!["(", "(", "1", "+", "2", ")", "*", "3", ")"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 9.0);
}

#[test]
fn test_bracket_errors() {
    let input = vec!["(", "2", "+", "3"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens);
    assert!(matches!(result, Err(Error::MismatchedBracket)));

    let input = vec!["2", "+", "3", ")"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens);
    assert!(matches!(result, Err(Error::MismatchedBracket)));

    let input = vec!["(", ")"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens);
    assert!(matches!(result, Err(Error::EmptyBrackets)));
}

#[test]
fn test_exponentiation() {
    let input = vec!["2", "^", "3"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 8.0);

    let input = vec!["3", "+", "2", "^", "3"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 11.0);

    let input = vec!["2", "^", "3", "^", "2"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 512.0);
}

#[test]
fn test_exponentiation_edge_cases() {
    let input = vec!["0", "^", "0"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 1.0);

    let input = vec!["999999", "^", "999999"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens);
    assert!(matches!(result, Err(Error::NotFinite)));
}

#[test]
fn test_combined_bodmas_expressions() {
    let input = vec!["(", "2", "+", "3", ")", "^", "2"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 25.0);

    let input = vec!["2", "^", "(", "1", "+", "2", ")", "*", "3"];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 24.0);

    let input = vec![
        "(", "3", "+", "2", ")", "*", "(", "2", "^", "3", "-", "1", ")",
    ];
    let tokens = parse_expression(input).unwrap();
    let result = evaluate_expression(&tokens).unwrap();
    assert_eq!(result, 35.0);
}
