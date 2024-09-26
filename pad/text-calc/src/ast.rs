//! Abstract Syntax Tree (AST) definitions for the calculator.
//!
//! This module defines the structures used to represent parsed expressions.

/// Represents an expression node in the AST.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A numeric literal.
    Number(f64),
    /// A number with an optional unit (e.g., `5 kg`).
    NumberWithUnit {
        /// The numeric value.
        value: f64,
        /// The unit of the value, if any.
        unit: Option<String>,
    },
    /// An expression with a unary operator (e.g., `-x`).
    UnaryOp {
        /// The unary operator.
        op: UnaryOp,
        /// The expression the operator is applied to.
        expr: Box<Expr>,
    },
    /// An expression with a binary operator (e.g., `x + y`).
    BinaryOp {
        /// The left-hand side expression.
        left: Box<Expr>,
        /// The binary operator.
        op: BinaryOp,
        /// The right-hand side expression.
        right: Box<Expr>,
    },
    /// A function call with a name and an argument (e.g., `sin(x)`).
    Function {
        /// The name of the function.
        name: String,
        /// The argument of the function.
        arg: Box<Expr>,
    },
    /// A unit conversion expression (e.g., `5 kg to lb`).
    Conversion {
        /// The expression to convert.
        expr: Box<Expr>,
        /// The target unit.
        target_unit: String,
    },
}

/// Represents a unary operator.
#[derive(Debug, Clone)]
pub enum UnaryOp {
    /// Unary plus (`+`).
    Plus,
    /// Unary minus (`-`).
    Minus,
}

/// Represents a binary operator.
#[derive(Debug, Clone)]
pub enum BinaryOp {
    /// Addition operator (`+`).
    Add,
    /// Subtraction operator (`-`).
    Subtract,
    /// Multiplication operator (`*`).
    Multiply,
    /// Division operator (`/`).
    Divide,
    /// Exponentiation operator (`^`).
    Power,
}

