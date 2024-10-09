use textcalculator::evaluate;

fn approximate(value: f64, size: i32) -> f64 {
    let size = 10_f64.powi(size);
    (value * size).floor() / size
}

#[test]
fn test_one_plus_one() {
    let result = evaluate("1 + 1").unwrap();
    assert_eq!(approximate(result, 2), approximate(2., 2));
}

#[test]
fn test_simple_expression() {
    let result = evaluate("3 + 4 * 2 / (1 - 5) ^ 2 ^ 3").unwrap();
    assert_eq!(approximate(result, 12), approximate(3.0001220703125, 12));
}

#[test]
fn test_function_evaluation() {
    let result = evaluate("sin(pi / 2)").unwrap();
    assert_eq!(approximate(result, 3), approximate(1.0, 3));
}

#[test]
fn test_bracket_usage() {
    let result = evaluate("[2 * {3 + (4 - 1)}] / 5").unwrap();
    assert_eq!(approximate(result, 4), approximate(2.4, 4));
}

#[test]
fn test_unary_minus() {
    let result = evaluate("-5 + 3 * -2").unwrap();
    assert_eq!(approximate(result, 2), approximate(-11.0, 2));
}
