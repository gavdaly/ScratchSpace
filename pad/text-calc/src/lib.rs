//! Calculator library module.
//!
//! This module provides functionality to parse and evaluate mathematical expressions,
//! including support for units, functions, and conversions.

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod parser;

use error::Result;

/// Evaluates a mathematical expression string and returns the result.
///
/// This function takes a mathematical expression as a string, parses it into an
/// Abstract Syntax Tree (AST), evaluates the AST, and returns the result as a
/// floating-point number.
///
/// # Arguments
///
/// * `expression` - A string slice that holds the mathematical expression to evaluate.
///
/// # Returns
///
/// * `Result<f64>` - A Result containing the evaluated value as an f64 if successful,
///   or an error if parsing or evaluation fails.
///
/// # Examples
///
/// ```
/// use text_calc::evaluate;
///
/// let result = evaluate("2 + 3 * 4");
/// assert_eq!(result, Ok(14.0));
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The expression cannot be parsed into a valid AST.
/// * The evaluation of the AST fails (e.g., division by zero, unknown function, etc.).

pub fn evaluate(expression: &str) -> Result<f64> {
    let ast = parser::parse_expression(expression)?;
    let result = evaluator::evaluate(&ast)?;
    Ok(result)
}
