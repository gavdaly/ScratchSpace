use crate::parser::Rule;
use pest::error::Error as PestError;
use std::fmt::Display;
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for the calculator library.
#[derive(Debug)]
pub enum Error {
    /// An error occurred during parsing.
    ParsingError(PestError<Rule>),
    /// An error occurred during evaluation.
    EvaluationError(String),
}

impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self {
        Error::ParsingError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingError(e) => write!(f, "Parsing error: {}", e),
            Error::EvaluationError(e) => write!(f, "Evaluation error: {}", e),
        }
    }
}
