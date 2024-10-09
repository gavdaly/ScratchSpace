use crate::parser::Rule;
use pest::error::Error as PestError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

/// Custom result type for the calculator library.
pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;

/// Custom error type for the calculator library.
#[derive(Debug)]
pub enum Error {
    /// An error occurred during parsing.
    ParsingError(Box<PestError<Rule>>),
    /// An error occurred during evaluation.
    EvaluationError(String),
}

impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self {
        Error::ParsingError(Box::new(error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingError(e) => write!(f, "Parsing error: {e}"),
            Error::EvaluationError(e) => write!(f, "Evaluation error: {e}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ParsingError(e) => Some(e),
            Error::EvaluationError(_) => None,
        }
    }
}
