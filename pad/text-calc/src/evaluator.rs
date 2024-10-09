//! Evaluator for the calculator AST.
//!
//! This module evaluates the AST and computes the result, handling units and conversions.

use crate::ast::{BinaryOp, Constant, Expr as Expression, Function, Group};
use crate::error::{Error, Result};
use std::f64::consts;

/// Evaluates the given expression and returns the computed result.
///
/// # Arguments
///
/// * `expr` - A reference to the Expression to be evaluated.
///
/// # Returns
///
/// * `Result<f64>` - The computed result as a float, or an error if evaluation fails.
///
/// # Errors
///
/// This function will return an error if:
/// * Division by zero is attempted
/// * Any other evaluation error occurs (e.g., invalid function arguments)
pub fn evaluate(expr: &Expression) -> Result<f64> {
    let value = match expr {
        Expression::Number(n) => *n,
        Expression::BinaryOp { left, op, right } => {
            let left_result = evaluate(left)?;
            let right_result = evaluate(right)?;

            match op {
                BinaryOp::Add => left_result + right_result,
                BinaryOp::Subtract => left_result - right_result,
                BinaryOp::Multiply => left_result * right_result,
                BinaryOp::Divide => {
                    if right_result == 0.0 {
                        return Err(Box::new(Error::EvaluationError(
                            "Division by zero error".to_string(),
                        )));
                    }
                    left_result / right_result
                }
                BinaryOp::Power => left_result.powf(right_result),
            }
        }
        Expression::Function { name, arg } => {
            let arg_result = evaluate(arg)?;
            match name {
                Function::Sin => arg_result.sin(),
                Function::Cos => arg_result.cos(),
                Function::Tan => arg_result.tan(),
                Function::Asin => arg_result.asin(),
                Function::Acos => arg_result.acos(),
                Function::Atan => arg_result.atan(),
                Function::Sinh => arg_result.sinh(),
                Function::Cosh => arg_result.cosh(),
                Function::Tanh => arg_result.tanh(),
                Function::Asinh => arg_result.asinh(),
                Function::Acosh => arg_result.acosh(),
                Function::Atanh => arg_result.atanh(),
                Function::Ln => arg_result.ln(),
                Function::Log10 => arg_result.log10(),
                Function::Sqrt => arg_result.sqrt(),
                Function::Exp => arg_result.exp(),
                Function::Abs => arg_result.abs(),
                Function::Ceil => arg_result.ceil(),
                Function::Floor => arg_result.floor(),
                Function::Round => arg_result.round(),
            }
        }
        Expression::Grouping(group) => match group {
            Group::Curly(expr) => evaluate(expr)?,
            Group::Square(expr) => evaluate(expr)?,
            Group::Paren(expr) => evaluate(expr)?,
        },
        Expression::Constant(constant) => match constant {
            Constant::Pi => consts::PI,
            Constant::E => consts::E,
        },
    };
    Ok(value)
}
