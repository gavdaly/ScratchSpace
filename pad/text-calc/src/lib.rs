//! Calculator library module.
//!
//! This module provides functionality to parse and evaluate mathematical expressions,
//! including support for units, functions, and conversions.

pub mod ast;
pub mod evaluator;
pub mod parser;

pub fn evaluate(expression: &str) -> Result<f64, String> {
    let ast = parser::parse_expression(expression)?;
    let result = evaluator::evaluate(&ast)?;
    Ok(result)
}
