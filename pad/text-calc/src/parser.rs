//! Parser for the calculator expressions.
//!
//! This module uses Pest to parse input strings into an AST.
use crate::ast::{BinaryOp, Expr, Function};
use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

/// Pest parser for the calculator, based on the grammar defined in `calculator.pest`.
#[derive(Parser)]
#[grammar = "calculator.pest"]
pub struct CalculatorParser;

/// Parses an expression string into an AST.
///
/// # Arguments
///
/// * `expression` - The expression string to parse.
///
/// # Returns
///
/// * `Ok(Expr)` - The parsed expression as an AST.
/// * `Err(String)` - An error message if parsing fails.
pub fn parse_expression(expression: &str) -> Result<Expr, String> {
    let pairs =
        CalculatorParser::parse(Rule::expr, expression).map_err(|e| format_pest_error(e))?;

    let pair = pairs.into_iter().next().unwrap();
    build_expr(pair)
}

/// Formats a Pest parsing error into a string.
///
/// # Arguments
///
/// * `error` - The Pest error to format.
///
/// # Returns
///
/// * A string containing the formatted error message.
fn format_pest_error(error: PestError<Rule>) -> String {
    format!("Parsing error: {}", error)
}

/// Recursively builds the AST from the parsed pairs.
///
/// # Arguments
///
/// * `pair` - A pair from the Pest parser.
///
/// # Returns
///
/// * `Ok(Expr)` - The constructed AST node.
/// * `Err(String)` - An error message if AST construction fails.
fn build_expr(pair: Pair<Rule>) -> Result<Expr, String> {
    match pair.as_rule() {
        Rule::expr | Rule::grouping => {
            let mut inner_rules = pair.into_inner();
            let first = build_expr(inner_rules.next().unwrap())?;

            let mut result = first;
            while let Some(next_pair) = inner_rules.next() {
                let operator = next_pair.as_str();
                let next_expr = build_expr(inner_rules.next().unwrap())?;
                result = match operator {
                    "+" => Expr::BinaryOp {
                        left: Box::new(result),
                        op: BinaryOp::Add,
                        right: Box::new(next_expr),
                    },
                    "-" => Expr::BinaryOp {
                        left: Box::new(result),
                        op: BinaryOp::Subtract,
                        right: Box::new(next_expr),
                    },
                    "*" => Expr::BinaryOp {
                        left: Box::new(result),
                        op: BinaryOp::Multiply,
                        right: Box::new(next_expr),
                    },
                    "/" => Expr::BinaryOp {
                        left: Box::new(result),
                        op: BinaryOp::Divide,
                        right: Box::new(next_expr),
                    },
                    "^" => Expr::BinaryOp {
                        left: Box::new(result),
                        op: BinaryOp::Power,
                        right: Box::new(next_expr),
                    },
                    _ => return Err(format!("Unknown operator: {}", operator)),
                };
            }
            Ok(result)
        }
        Rule::function => {
            let mut inner_rules = pair.into_inner();
            let arg = build_expr(inner_rules.next().unwrap())?;
            Ok(Expr::Function {
                name: Function::Sin,
                arg: Box::new(arg),
            })
        }
        _ => Err(format!("Unhandled rule: {:?}", pair.as_rule())),
    }
}
