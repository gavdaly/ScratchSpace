//! Main program for the calculator.
//!
//! This module provides a command-line interface to input expressions and display results.
//! Supports CLI mode, TUI mode, and scripting mode.

mod ast;
mod error;
mod evaluator;
mod parser;

use clap::{Parser, Subcommand};

use crate::evaluator::evaluate;
use crate::parser::parse_expression;

/// Command-line arguments for the calculator.
#[derive(Parser, Debug)]
#[command(
    name = "calc",
    about = "A command-line calculator with unit conversions and scientific functions."
)]
struct Args {
    /// Mathematical expression to evaluate (if provided, evaluates the expression directly)
    #[arg()]
    expression: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// Supported subcommands for the calculator.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Evaluate expressions from a script file
    Eval {
        /// Path to the script file
        script: String,
    },
}

fn main() {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Eval { script } => match run_script(&script) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            },
        }
    } else if let Some(expression) = args.expression {
        match evaluate_expression(&expression) {
            Ok(result) => println!("Result: {result}"),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        // TUI Mode: Enter interactive calculator mode
        run_tui();
    }
}

/// Evaluates a single expression string.
/// # Arguments
/// * `expression` - The expression string to evaluate.
/// # Returns
/// * `Ok(ValueWithUnit)` - The result of the evaluation.
/// * `Err(String)` - An error message if evaluation fails.
fn evaluate_expression(expression: &str) -> Result<f64, String> {
    match parser::parse_expression(expression) {
        Ok(ast) => {
            evaluator::evaluate(&ast).map_err(|e| format!("Error evaluating '{expression}': {e}"))
        }
        Err(e) => Err(format!("Error parsing expression '{}': {}", expression, e)),
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Runs the calculator in scripting mode, evaluating expressions from a file.
///
/// # Arguments
///
/// * `script_path` - The path to the script file.
///
/// # Returns
///
/// * `Ok(())` - If the script was executed successfully.
/// * `Err(String)` - An error message if execution fails.
fn run_script(script_path: &str) -> Result<(), String> {
    let file = File::open(script_path).map_err(|e| format!("Failed to open script file: {}", e))?;
    let reader = BufReader::new(file);

    for (line_number, line_result) in reader.lines().enumerate() {
        let line_number = line_number + 1; // Line numbers start at 1
        let line = line_result.map_err(|e| format!("Error reading script file: {}", e))?;
        let expression = line.trim();

        if expression.is_empty() || expression.starts_with('#') {
            // Skip empty lines and comments
            continue;
        }
        let expression =
            parse_expression(expression).map_err(|e| format!("Error parsing expression: {}", e))?;
        match evaluate(&expression) {
            Ok(result) => {
                print!("Line {line_number}: {result}");
            }
            Err(e) => {
                eprintln!("Error on line {}: {}", line_number, e);
            }
        }
    }

    Ok(())
}

/// Runs the calculator in interactive mode (TUI).
fn run_tui() {
    use reedline::{DefaultPrompt, Reedline};
    let mut _line_editor = Reedline::create();
    let _prompt = DefaultPrompt::default();

    unimplemented!("TUI mode is not yet implemented.");
}
