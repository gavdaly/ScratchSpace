//! Evaluator for the calculator AST.
//!
//! This module evaluates the AST and computes the result, handling units and conversions.

use crate::ast::{Expr, UnaryOp, BinaryOp};
use std::collections::HashMap;
use std::f64::consts::PI;

/// Represents a value with an optional unit.
#[derive(Debug, Clone)]
pub struct ValueWithUnit {
    /// The numeric value.
    pub value: f64,
    /// The unit of the value, if any.
    pub unit: Option<String>,
}

/// Evaluates an expression AST and computes the result.
///
/// # Arguments
///
/// * `expr` - The expression AST to evaluate.
///
/// # Returns
///
/// * `Ok(ValueWithUnit)` - The computed value with unit.
/// * `Err(String)` - An error message if evaluation fails.
pub fn evaluate(expr: &Expr) -> Result<ValueWithUnit, String> {
    match expr {
        Expr::Number(n) => Ok(ValueWithUnit {
            value: *n,
            unit: None,
        }),
        Expr::NumberWithUnit { value, unit } => Ok(ValueWithUnit {
            value: *value,
            unit: unit.clone(),
        }),
        Expr::UnaryOp { op, expr } => {
            let result = evaluate(expr)?;
            let value = match op {
                UnaryOp::Plus => result.value,
                UnaryOp::Minus => -result.value,
            };
            Ok(ValueWithUnit {
                value,
                unit: result.unit,
            })
        }
        Expr::BinaryOp { left, op, right } => {
            let left_result = evaluate(left)?;
            let right_result = evaluate(right)?;

            // For simplicity, we'll assume units must match for addition and subtraction
            match op {
                BinaryOp::Add | BinaryOp::Subtract => {
                    if left_result.unit != right_result.unit {
                        return Err("Unit mismatch in addition or subtraction".to_string());
                    }
                    let value = match op {
                        BinaryOp::Add => left_result.value + right_result.value,
                        BinaryOp::Subtract => left_result.value - right_result.value,
                        _ => unreachable!(),
                    };
                    Ok(ValueWithUnit {
                        value,
                        unit: left_result.unit.clone(),
                    })
                }
                BinaryOp::Multiply | BinaryOp::Divide => {
                    // For simplicity, we'll not handle unit multiplication/division in detail
                    let value = match op {
                        BinaryOp::Multiply => left_result.value * right_result.value,
                        BinaryOp::Divide => {
                            if right_result.value == 0.0 {
                                return Err("Division by zero error".to_string());
                            }
                            left_result.value / right_result.value
                        }
                        _ => unreachable!(),
                    };
                    // Units handling can be more complex; we'll ignore units in multiplication/division
                    Ok(ValueWithUnit {
                        value,
                        unit: None,
                    })
                }
                BinaryOp::Power => {
                    let value = left_result.value.powf(right_result.value);
                    Ok(ValueWithUnit {
                        value,
                        unit: left_result.unit.clone(),
                    })
                }
            }
        }
        Expr::Function { name, arg } => {
            let arg_result = evaluate(arg)?;
            let value = match name.as_str() {
                "sin" => arg_result.value.sin(),
                "cos" => arg_result.value.cos(),
                "tan" => arg_result.value.tan(),
                "asin" => arg_result.value.asin(),
                "acos" => arg_result.value.acos(),
                "atan" => arg_result.value.atan(),
                "sqrt" => arg_result.value.sqrt(),
                "log" => arg_result.value.log10(),
                "ln" => arg_result.value.ln(),
                "exp" => arg_result.value.exp(),
                "abs" => arg_result.value.abs(),
                "ceil" => arg_result.value.ceil(),
                "floor" => arg_result.value.floor(),
                "round" => arg_result.value.round(),
                "trunc" => arg_result.value.trunc(),
                "fract" => arg_result.value.fract(),
                "radians" => arg_result.value.to_radians(),
                "degrees" => arg_result.value.to_degrees(),
                "pi" => PI,
                "e" => std::f64::consts::E,
                _ => return Err(format!("Unknown function: {}", name)),
            };
            Ok(ValueWithUnit {
                value,
                unit: None, // Functions return unitless results
            })
        }
        Expr::Conversion { expr, target_unit } => {
            let value_with_unit = evaluate(expr)?;
            if let Some(source_unit) = value_with_unit.unit {
                let converted_value = convert_units(value_with_unit.value, &source_unit, target_unit)?;
                Ok(ValueWithUnit {
                    value: converted_value,
                    unit: Some(target_unit.clone()),
                })
            } else {
                Err("Cannot convert a unitless value".to_string())
            }
        }
    }
}

/// Converts a value from one unit to another.
///
/// # Arguments
///
/// * `value` - The numeric value to convert.
/// * `from_unit` - The source unit.
/// * `to_unit` - The target unit.
///
/// # Returns
///
/// * `Ok(f64)` - The converted value.
/// * `Err(String)` - An error message if conversion fails.
fn convert_units(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // We'll define a simple conversion map
    let mut conversion_factors: HashMap<(&str, &str), f64> = HashMap::new();

    // Length units
    conversion_factors.insert(("m", "ft"), 3.28084);
    conversion_factors.insert(("ft", "m"), 0.3048);
    conversion_factors.insert(("km", "mi"), 0.621371);
    conversion_factors.insert(("mi", "km"), 1.60934);

    // Mass units
    conversion_factors.insert(("kg", "lb"), 2.20462);
    conversion_factors.insert(("lb", "kg"), 0.453592);

    // Temperature units (special handling)
    if from_unit == "C" && to_unit == "F" {
        return Ok(value * 9.0 / 5.0 + 32.0);
    } else if from_unit == "F" && to_unit == "C" {
        return Ok((value - 32.0) * 5.0 / 9.0);
    }

    if let Some(factor) = conversion_factors.get(&(from_unit, to_unit)) {
        Ok(value * factor)
    } else {
        Err(format!(
            "Conversion from '{}' to '{}' not supported",
            from_unit, to_unit
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, UnaryOp, BinaryOp};

    #[test]
    fn test_evaluate_number() {
        let expr = Expr::Number(42.0);
        let result = evaluate(&expr).unwrap();
        assert_eq!(result.value, 42.0);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_evaluate_addition() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(1.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };
        let result = evaluate(&expr).unwrap();
        assert_eq!(result.value, 3.0);
    }

    #[test]
    fn test_evaluate_conversion() {
        let expr = Expr::Conversion {
            expr: Box::new(Expr::NumberWithUnit {
                value: 5.0,
                unit: Some("kg".to_string()),
            }),
            target_unit: "lb".to_string(),
        };
        let result = evaluate(&expr).unwrap();
        assert_eq!(result.unit.unwrap(), "lb");
        assert!((result.value - 11.0231).abs() < 0.0001);
    }

    #[test]
    fn test_evaluate_function() {
        let expr = Expr::Function {
            name: "sin".to_string(),
            arg: Box::new(Expr::Number(0.0)),
        };
        let result = evaluate(&expr).unwrap();
        assert_eq!(result.value, 0.0);
    }
}
