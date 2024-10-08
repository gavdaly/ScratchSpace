//! Calculator library module.
//!
//! This module provides functionality to parse and evaluate mathematical expressions,
//! including support for units, functions, and conversions.

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod parser;

use error::Result;

pub fn evaluate(expression: &str) -> Result<f64> {
    let ast = parser::parse_expression(expression)?;
    let result = evaluator::evaluate(&ast)?;
    Ok(result)
}
