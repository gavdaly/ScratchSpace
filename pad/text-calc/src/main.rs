//! Main program for the calculator.
//!
//! This module provides a command-line interface to input expressions and display results.
//! Supports CLI mode, TUI mode, and scripting mode.

mod ast;
mod evaluator;
mod parser;

use clap::{Parser, Subcommand};
// use evaluator::ValueWithUnit;

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
            Commands::Eval { script } => {
                // Scripting Mode: Evaluate expressions from a script file
                match run_script(&script) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    } else if let Some(_expression) = args.expression {
        // CLI Mode: Evaluate the expression provided as an argument
        // match evaluate_expression(&expression) {
        //     Ok(result) => print_result(&result),
        //     Err(e) => eprintln!("Error: {}", e),
        // }
    } else {
        // TUI Mode: Enter interactive calculator mode
        run_tui();
    }
}

// Evaluates a single expression string.

// # Arguments

// * `expression` - The expression string to evaluate.

// # Returns

// * `Ok(ValueWithUnit)` - The result of the evaluation.
// * `Err(String)` - An error message if evaluation fails.
// fn evaluate_expression(expression: &str) -> Result<ValueWithUnit, String> {
//     match parser::parse_expression(expression) {
//         Ok(ast) => evaluator::evaluate(&ast)
//             .map_err(|e| format!("Error evaluating '{}': {}", expression, e)),
//         Err(e) => Err(format!("Error parsing expression '{}': {}", expression, e)),
//     }
// }

// Prints the result, including the unit if present.

// # Arguments

// * `result` - The result to print.
// fn print_result(result: &ValueWithUnit) {
//     if let Some(unit) = &result.unit {
//         println!("Result: {} {}", result.value, unit);
//     } else {
//         println!("Result: {}", result.value);
//     }
// }

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

        // match evaluate_expression(expression) {
        //     Ok(result) => {
        //         print!("Line {}: ", line_number);
        //         print_result(&result);
        //     }
        //     Err(e) => {
        //         eprintln!("Error on line {}: {}", line_number, e);
        //     }
        // }
    }

    Ok(())
}

/// Runs the calculator in interactive mode (TUI).
fn run_tui() {
    use reedline::{DefaultPrompt, Reedline};
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(_input) => {
                // let expression = input.trim();
                // if expression.eq_ignore_ascii_case("exit")
                //     || expression.eq_ignore_ascii_case("quit")
                // {
                //     break;
                // }

                // match evaluate_expression(expression) {
                //     Ok(result) => print_result(&result),
                //     Err(e) => eprintln!("{}", e),
                // }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}
