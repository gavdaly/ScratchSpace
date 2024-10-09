//! Parser for the calculator expressions.
//!
//! This module uses Pest to parse input strings into an AST.
use crate::ast::{BinaryOp, Constant, Expr, Function, Group};
use crate::error::{Error, Result};
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
pub fn parse_expression(expression: &str) -> Result<Expr> {
    let pairs =
        CalculatorParser::parse(Rule::expr, expression).map_err(|e| Error::ParsingError(e))?;

    let pair = pairs.into_iter().next().unwrap();
    build_expr(pair)
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
fn build_expr(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::expr => {
            let mut inner_rules = pair.into_inner();
            let mut result = build_expr(inner_rules.next().unwrap())?;

            while let Some(next_pair) = inner_rules.next() {
                let &operator = &next_pair.as_str();
                let rule = next_pair.as_rule();
                dbg!(&operator, &rule);
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
                    _ => {
                        return Err(Error::EvaluationError(format!(
                            "Unknown operator: {:?}",
                            operator
                        )))
                    }
                };
            }
            Ok(result)
        }
        Rule::float => {
            let value: f64 = pair.as_span().as_str().parse().unwrap();
            Ok(Expr::Number(value))
        }
        Rule::integer => {
            let value: f64 = pair.as_span().as_str().parse().unwrap();
            Ok(Expr::Number(value))
        }

        Rule::constant => match &pair.as_rule() {
            Rule::e => Ok(Expr::Constant(Constant::E)),
            Rule::pi => Ok(Expr::Constant(Constant::Pi)),
            _ => {
                return Err(Error::EvaluationError(format!(
                    "Unknown constant: {}",
                    pair.as_str()
                )))
            }
        },

        Rule::function => {
            let rule = &pair.as_rule();
            let mut inner_rules = pair.into_inner();
            let arg = build_expr(inner_rules.next().unwrap())?;
            match rule {
                Rule::sin => Ok(Expr::Function {
                    name: Function::Sin,
                    arg: Box::new(arg),
                }),
                Rule::cos => Ok(Expr::Function {
                    name: Function::Cos,
                    arg: Box::new(arg),
                }),
                Rule::tan => Ok(Expr::Function {
                    name: Function::Tan,
                    arg: Box::new(arg),
                }),
                Rule::sinh => Ok(Expr::Function {
                    name: Function::Sinh,
                    arg: Box::new(arg),
                }),
                Rule::cosh => Ok(Expr::Function {
                    name: Function::Cosh,
                    arg: Box::new(arg),
                }),
                Rule::tanh => Ok(Expr::Function {
                    name: Function::Tanh,
                    arg: Box::new(arg),
                }),
                Rule::asin => Ok(Expr::Function {
                    name: Function::Asin,
                    arg: Box::new(arg),
                }),
                Rule::acos => Ok(Expr::Function {
                    name: Function::Acos,
                    arg: Box::new(arg),
                }),
                Rule::atan => Ok(Expr::Function {
                    name: Function::Atan,
                    arg: Box::new(arg),
                }),
                Rule::asinh => Ok(Expr::Function {
                    name: Function::Asinh,
                    arg: Box::new(arg),
                }),
                Rule::acosh => Ok(Expr::Function {
                    name: Function::Acosh,
                    arg: Box::new(arg),
                }),
                Rule::atanh => Ok(Expr::Function {
                    name: Function::Atanh,
                    arg: Box::new(arg),
                }),
                Rule::sqrt => Ok(Expr::Function {
                    name: Function::Sqrt,
                    arg: Box::new(arg),
                }),
                Rule::exp => Ok(Expr::Function {
                    name: Function::Exp,
                    arg: Box::new(arg),
                }),
                Rule::ln => Ok(Expr::Function {
                    name: Function::Ln,
                    arg: Box::new(arg),
                }),
                Rule::log => Ok(Expr::Function {
                    name: Function::Log10,
                    arg: Box::new(arg),
                }),
                Rule::abs => Ok(Expr::Function {
                    name: Function::Abs,
                    arg: Box::new(arg),
                }),
                Rule::ceil => Ok(Expr::Function {
                    name: Function::Ceil,
                    arg: Box::new(arg),
                }),
                Rule::floor => Ok(Expr::Function {
                    name: Function::Floor,
                    arg: Box::new(arg),
                }),
                Rule::round => Ok(Expr::Function {
                    name: Function::Round,
                    arg: Box::new(arg),
                }),
                _ => Err(Error::EvaluationError(format!(
                    "Unknown function: {:?}",
                    rule
                ))),
            }
        }
        Rule::grouping => {
            let rule = &pair.as_rule();
            let mut inner_rules = pair.into_inner();
            let arg = build_expr(inner_rules.next().unwrap())?;

            match rule {
                Rule::brackets => Ok(Expr::Grouping(Group::Paren(Box::new(arg)))),
                Rule::square => Ok(Expr::Grouping(Group::Square(Box::new(arg)))),
                Rule::curly => Ok(Expr::Grouping(Group::Curly(Box::new(arg)))),
                _ => Err(Error::EvaluationError(format!(
                    "Unknown grouping: {:?}",
                    rule
                ))),
            }
        }
        _ => Err(Error::EvaluationError(format!(
            "Unhandled rule: {:?}",
            pair.as_rule()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let expr = parse_expression("1 + 2").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_parse_expression_with_brackets() {
        let expr = parse_expression("(1 + 2) * 3").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::Grouping(Group::Paren(Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(2.0))
                })))),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_expression_with_function() {
        let expr = parse_expression("sin(1)").unwrap();
        assert_eq!(
            expr,
            Expr::Function {
                name: Function::Sin,
                arg: Box::new(Expr::Number(1.0)),
            }
        );
    }

    #[test]
    fn test_parse_expression_with_power() {
        let expr = parse_expression("2^3").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Power,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_expression_with_constants() {
        let expr = parse_expression("pi + e").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                left: Box::new(Expr::Constant(Constant::Pi)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Constant(Constant::E)),
            }
        );
    }
}
