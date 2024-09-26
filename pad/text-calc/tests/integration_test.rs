use calculator::{parser, evaluator};
use calculator::evaluator::ValueWithUnit;

#[test]
fn test_simple_expression() {
    let expr = parser::parse_expression("3 + 4 * 2 / (1 - 5) ^ 2 ^ 3").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert!((result.value - 3.0001220703125).abs() < 0.0001);
}

#[test]
fn test_unit_conversion() {
    let expr = parser::parse_expression("5 kg to lb").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert_eq!(result.unit.unwrap(), "lb");
    assert!((result.value - 11.0231).abs() < 0.0001);
}

#[test]
fn test_temperature_conversion() {
    let expr = parser::parse_expression("100 C to F").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert_eq!(result.unit.unwrap(), "F");
    assert_eq!(result.value, 212.0);
}

#[test]
fn test_function_evaluation() {
    let expr = parser::parse_expression("sin(pi / 2)").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert!((result.value - 1.0).abs() < 0.0001);
}

#[test]
fn test_error_handling() {
    let expr = parser::parse_expression("5 kg + 2 lb");
    assert!(expr.is_ok());
    let eval_result = evaluator::evaluate(&expr.unwrap());
    assert!(eval_result.is_err());
}

#[test]
fn test_bracket_usage() {
    let expr = parser::parse_expression("[2 * {3 + (4 - 1)}] / 5").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert!((result.value - 2.4).abs() < 0.0001);
}

#[test]
fn test_unary_minus() {
    let expr = parser::parse_expression("-5 + 3 * -2").unwrap();
    let result = evaluator::evaluate(&expr).unwrap();
    assert_eq!(result.value, -11.0);
}
