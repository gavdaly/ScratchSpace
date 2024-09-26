//! Parser for the calculator expressions.
//!
//! This module uses Pest to parse input strings into an AST.

use pest::Parser;
use pest::iterators::Pair;
use pest::error::Error as PestError;
use crate::ast::{Expr, UnaryOp, BinaryOp};

/// Pest parser for the calculator, based on the grammar defined in `calculator.pest`.
#[derive(pest_derive::Parser)]
#[grammar = "calculator.pest"]
pub struct CalculatorParser;

use crate::parser::Rule;

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
    let parse_result = CalculatorParser::parse(Rule::expression, expression);
    match parse_result {
        Ok(mut pairs) => build_expr(pairs.next().unwrap()),
        Err(e) => Err(format_pest_error(e)),
    }
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
        Rule::conversion => {
            let mut inner_rules = pair.into_inner();
            let value_expr = build_expr(inner_rules.next().unwrap())?;
            let target_unit = inner_rules.next().unwrap().as_str().to_string();
            Ok(Expr::Conversion {
                expr: Box::new(value_expr),
                target_unit,
            })
        }
        Rule::expression | Rule::term | Rule::factor | Rule::power | Rule::unary | Rule::primary | Rule::grouping => {
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
        Rule::number_with_unit => {
            let mut inner_rules = pair.into_inner();
            let number_pair = inner_rules.next().unwrap();
            let value = number_pair.as_str().parse::<f64>().map_err(|e| e.to_string())?;
            let unit = if let Some(unit_pair) = inner_rules.next() {
                Some(unit_pair.as_str().to_string())
            } else {
                None
            };
            Ok(Expr::NumberWithUnit { value, unit })
        }
        Rule::number => {
            let num = pair.as_str().parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr::Number(num))
        }
        Rule::function => {
            let mut inner_rules = pair.into_inner();
            let name = inner_rules.next().unwrap().as_str().to_string();
            let arg = build_expr(inner_rules.next().unwrap())?;
            Ok(Expr::Function {
                name,
                arg: Box::new(arg),
            })
        }
        Rule::unary => {
            let mut inner_rules = pair.into_inner();
            let mut op_signs = Vec::new();

            while let Some(next) = inner_rules.peek() {
                if next.as_rule() == Rule::primary
                    || next.as_rule() == Rule::grouping
                    || next.as_rule() == Rule::number_with_unit
                {
                    break;
                }
                op_signs.push(inner_rules.next().unwrap().as_str());
            }

            let expr = build_expr(inner_rules.next().unwrap())?;

            let mut result = expr;
            for op in op_signs.into_iter().rev() {
                let op = match op {
                    "+" => UnaryOp::Plus,
                    "-" => UnaryOp::Minus,
                    _ => return Err(format!("Unknown unary operator: {}", op)),
                };
                result = Expr::UnaryOp {
                    op,
                    expr: Box::new(result),
                };
            }
            Ok(result)
        }
        Rule::primary => build_expr(pair.into_inner().next().unwrap()),
        Rule::grouping => build_expr(pair.into_inner().next().unwrap()),
        _ => Err(format!("Unhandled rule: {:?}", pair.as_rule())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expr = parse_expression("42").unwrap();
        if let Expr::Number(n) = expr {
            assert_eq!(n, 42.0);
        } else {
            panic!("Expected Expr::Number");
        }
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_expression("1 + 2").unwrap();
        if let Expr::BinaryOp { op, .. } = expr {
            if let BinaryOp::Add = op {
                // Test passed
            } else {
                panic!("Expected BinaryOp::Add");
            }
        } else {
            panic!("Expected Expr::BinaryOp");
        }
    }

    #[test]
    fn test_parse_conversion() {
        let expr = parse_expression("5 kg to lb").unwrap();
        if let Expr::Conversion { .. } = expr {
            // Test passed
        } else {
            panic!("Expected Expr::Conversion");
        }
    }

    #[test]
    fn test_parse_function() {
        let expr = parse_expression("sin(pi / 2)").unwrap();
        if let Expr::Function { name, .. } = expr {
            assert_eq!(name, "sin");
        } else {
            panic!("Expected Expr::Function");
        }
    }
}
