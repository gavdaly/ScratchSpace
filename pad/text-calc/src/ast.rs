//! Abstract Syntax Tree (AST) definitions for the calculator.
//!
//! This module defines the structures used to represent parsed expressions.

/// Represents an expression node in the AST.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A numeric literal.
    Number(f64),
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
        name: Function,
        /// The argument of the function.
        arg: Box<Expr>,
    },
    Grouping {
        group: Group,
    },
}

#[derive(Debug, Clone)]
pub enum Group {
    Curly(Box<Expr>),
    Square(Box<Expr>),
    Paren(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Sqrt,
    Exp,
    Ln,
    Log10,
    Abs,
    Ceil,
    Floor,
    Round,
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
