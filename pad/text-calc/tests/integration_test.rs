use textcalculator::evaluate;

#[test]
fn test_one_plus_one() {
    let result = evaluate("1 + 1").unwrap();
    assert!((result - 2.).abs() < 0.0001);
}

#[test]
fn test_simple_expression() {
    let result = evaluate("3 + 4 * 2 / (1 - 5) ^ 2 ^ 3").unwrap();
    assert!((result - 3.0001220703125).abs() < 0.0001);
}

#[test]
fn test_function_evaluation() {
    let result = evaluate("sin(pi / 2)").unwrap();
    assert!((result - 1.0).abs() < 0.0001);
}

#[test]
fn test_bracket_usage() {
    let result = evaluate("[2 * {3 + (4 - 1)}] / 5").unwrap();
    assert!((result - 2.4).abs() < 0.0001);
}

#[test]
fn test_unary_minus() {
    let result = evaluate("-5 + 3 * -2").unwrap();
    assert_eq!(result, -11.0);
}
