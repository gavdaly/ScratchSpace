//! Evaluator for the calculator AST.
//!
//! This module evaluates the AST and computes the result, handling units and conversions.

/// Represents a value with an optional unit.
#[derive(Debug, Clone)]
pub struct ValueWithUnit {
    /// The numeric value.
    pub value: f64,
    /// The unit of the value, if any.
    pub unit: Option<String>,
}

// Evaluates an expression AST and computes the result.

// # Arguments

// * `expr` - The expression AST to evaluate.

// # Returns

// * `Ok(ValueWithUnit)` - The computed value with unit.
// * `Err(String)` - An error message if evaluation fails.
// pub fn evaluate(expr: &Expr) -> Result<ValueWithUnit, String> {
//     match expr {
//         Expr::Number(n) => Ok(ValueWithUnit {
//             value: *n,
//             unit: None,
//         }),
//         Expr::BinaryOp { left, op, right } => {
//             let left_result = evaluate(left)?;
//             let right_result = evaluate(right)?;

//             // For simplicity, we'll assume units must match for addition and subtraction
//             match op {
//                 BinaryOp::Add | BinaryOp::Subtract => {
//                     if left_result.unit != right_result.unit {
//                         return Err("Unit mismatch in addition or subtraction".to_string());
//                     }
//                     let value = match op {
//                         BinaryOp::Add => left_result.value + right_result.value,
//                         BinaryOp::Subtract => left_result.value - right_result.value,
//                         _ => unreachable!(),
//                     };
//                     Ok(ValueWithUnit {
//                         value,
//                         unit: left_result.unit.clone(),
//                     })
//                 }
//                 BinaryOp::Multiply | BinaryOp::Divide => {
//                     // For simplicity, we'll not handle unit multiplication/division in detail
//                     let value = match op {
//                         BinaryOp::Multiply => left_result.value * right_result.value,
//                         BinaryOp::Divide => {
//                             if right_result.value == 0.0 {
//                                 return Err("Division by zero error".to_string());
//                             }
//                             left_result.value / right_result.value
//                         }
//                         _ => unreachable!(),
//                     };
//                     // Units handling can be more complex; we'll ignore units in multiplication/division
//                     Ok(ValueWithUnit { value, unit: None })
//                 }
//                 BinaryOp::Power => {
//                     let value = left_result.value.powf(right_result.value);
//                     Ok(ValueWithUnit {
//                         value,
//                         unit: left_result.unit.clone(),
//                     })
//                 }
//             }
//         }
//         Expr::Function { name, arg } => {
//             let arg_result = evaluate(arg)?;
//             let value = match name {
//                 Function::Sin => arg_result.value.sin(),
//                 Function::Cos => arg_result.value.cos(),
//                 Function::Tan => arg_result.value.tan(),
//                 Function::Asin => arg_result.value.asin(),
//                 Function::Acos => arg_result.value.acos(),
//                 Function::Atan => arg_result.value.atan(),
//                 Function::Sinh => arg_result.value.sinh(),
//                 Function::Cosh => arg_result.value.cosh(),
//                 Function::Tanh => arg_result.value.tanh(),
//                 Function::Asinh => arg_result.value.asinh(),
//                 Function::Acosh => arg_result.value.acosh(),
//                 Function::Atanh => arg_result.value.atanh(),
//                 Function::Sqrt => arg_result.value.sqrt(),
//                 Function::Log10 => arg_result.value.log10(),
//                 Function::Ln => arg_result.value.ln(),
//                 Function::Exp => arg_result.value.exp(),
//                 Function::Abs => arg_result.value.abs(),
//                 Function::Ceil => arg_result.value.ceil(),
//                 Function::Floor => arg_result.value.floor(),
//                 Function::Round => arg_result.value.round(),
//             };
//             Ok(ValueWithUnit {
//                 value,
//                 unit: None, // Functions return unitless results
//             })
//         }
//     }
// }

// Converts a value from one unit to another.

// # Arguments

// * `value` - The numeric value to convert.
// * `from_unit` - The source unit.
// * `to_unit` - The target unit.

// # Returns

// * `Ok(f64)` - The converted value.
// * `Err(String)` - An error message if conversion fails.
// fn convert_units(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
//     // We'll define a simple conversion map
//     let mut conversion_factors: HashMap<(&str, &str), f64> = HashMap::new();

//     // Length units
//     conversion_factors.insert(("m", "ft"), 3.28084);
//     conversion_factors.insert(("ft", "m"), 0.3048);
//     conversion_factors.insert(("km", "mi"), 0.621371);
//     conversion_factors.insert(("mi", "km"), 1.60934);

//     // Mass units
//     conversion_factors.insert(("kg", "lb"), 2.20462);
//     conversion_factors.insert(("lb", "kg"), 0.453592);

//     // Temperature units (special handling)
//     if from_unit == "C" && to_unit == "F" {
//         return Ok(value * 9.0 / 5.0 + 32.0);
//     } else if from_unit == "F" && to_unit == "C" {
//         return Ok((value - 32.0) * 5.0 / 9.0);
//     }

//     if let Some(factor) = conversion_factors.get(&(from_unit, to_unit)) {
//         Ok(value * factor)
//     } else {
//         Err(format!(
//             "Conversion from '{}' to '{}' not supported",
//             from_unit, to_unit
//         ))
//     }
// }
