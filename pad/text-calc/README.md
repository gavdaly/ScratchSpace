# TextCalc

**TextCalc** is a versatile command-line calculator written in Rust, featuring:

- **Mathematical Expression Evaluation**: Supports complex expressions with proper operator precedence.
- **Unit Conversions**: Convert between units like kilograms to pounds, kilometers to miles, Celsius to Fahrenheit, and more.
- **Scientific Functions**: Includes functions like `sin`, `cos`, `tan`, `sqrt`, `log`, and others.
- **Scripting Mode**: Evaluate expressions from a script file.
- **Interactive Mode (TUI)**: Provides an interactive shell for continuous calculations.
- **Support for Various Brackets**: Use `()`, `[]`, or `{}` for grouping expressions.
- **Enhanced Error Reporting**: Detailed parsing and evaluation errors with line numbers and descriptions.

---

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [CLI Mode](#cli-mode)
  - [Interactive Mode (TUI)](#interactive-mode-tui)
  - [Scripting Mode](#scripting-mode)
- [Features](#features)
  - [Mathematical Operations](#mathematical-operations)
  - [Unit Conversions](#unit-conversions)
  - [Functions](#functions)
  - [Brackets and Grouping](#brackets-and-grouping)
  - [Error Reporting](#error-reporting)
- [Examples](#examples)
- [Building from Source](#building-from-source)
- [Running Tests](#running-tests)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## Installation

### Pre-built Binaries

*Coming soon!* Pre-built binaries will be provided for major platforms.

### From Source

You can build TextCalc from source using Rust's package manager, Cargo.

#### Prerequisites

- **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/).

#### Steps

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/textcalc.git
   cd textcalc
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

   The executable will be located at `target/release/textcalc`.

3. **(Optional) Install**

   To install TextCalc globally:

   ```bash
   cargo install --path .
   ```

---

## Usage

TextCalc can be used in three modes:

### CLI Mode

Evaluate an expression directly from the command line.

```bash
textcalc "expression"
```

**Example:**

```bash
textcalc "5 kg to lb"
```

**Output:**

```
Result: 11.0231 lb
```

### Interactive Mode (TUI)

Start an interactive session.

```bash
textcalc
```

**Example Session:**

```
> 1 + 1
Result: 2
> sin(pi / 2)
Result: 1
> exit
```

Exit the interactive mode by typing `exit` or `quit`.

### Scripting Mode

Evaluate expressions from a script file.

```bash
textcalc eval script.txt
```

**Example `script.txt`:**

```plaintext
# Sample script for TextCalc

1 + 2 * 3
10 km to mi
sin(pi / 2)
invalid expression
5 kg + 2 lb
```

**Output:**

```
Line 3: Result: 7
Line 4: Result: 6.21371 mi
Line 5: Result: 1
Error on line 6: Error parsing expression 'invalid expression': Parsing error: --> 1:1
  |
1 | invalid expression
  | ^--- Expected valid expression
Error on line 7: Error evaluating '5 kg + 2 lb': Unit mismatch in addition or subtraction
```

---

## Features

### Mathematical Operations

- **Basic Arithmetic**: Addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`).
- **Exponentiation**: Power operator (`^`).
- **Operator Precedence**: Correct order of operations is enforced.

### Unit Conversions

Convert values between different units.

**Supported Units:**

- **Length**: `m`, `ft`, `km`, `mi`.
- **Mass**: `kg`, `lb`.
- **Temperature**: `C`, `F`.

**Usage:**

```plaintext
value source_unit to target_unit
```

**Example:**

```bash
textcalc "100 C to F"
```

**Output:**

```
Result: 212 F
```

### Functions

**Supported Functions:**

- Trigonometric: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- Exponential and Logarithmic: `exp`, `ln`, `log`
- Other: `sqrt`, `abs`, `ceil`, `floor`, `round`, `trunc`, `fract`
- Constants: `pi`, `e`

**Example:**

```bash
textcalc "sqrt(16) + log(100)"
```

**Output:**

```
Result: 6
```

### Brackets and Grouping

TextCalc supports grouping expressions using:

- Parentheses: `()`
- Square Brackets: `[]`
- Curly Braces: `{}`

**Example:**

```bash
textcalc "[2 * {3 + (4 - 1)}] / 5"
```

**Output:**

```
Result: 2.4
```

### Error Reporting

- **Parsing Errors**: Detailed messages with line and column numbers.
- **Evaluation Errors**: Descriptive messages indicating the cause of the error.
- **Scripting Mode**: Errors include line numbers from the script file.

---

## Examples

**Complex Expression:**

```bash
textcalc "3 + 4 * 2 / (1 - 5) ^ 2 ^ 3"
```

**Output:**

```
Result: 3.0001220703125
```

**Unit Conversion with Calculation:**

```bash
textcalc "(5 kg + 3 kg) to lb"
```

**Output:**

```
Result: 17.63696 lb
```

**Using Functions and Constants:**

```bash
textcalc "sin(pi / 4)"
```

**Output:**

```
Result: 0.7071067811865475
```

---

## Building from Source

See the [Installation](#installation) section for instructions on building TextCalc from source.

---

## Running Tests

TextCalc includes unit tests and integration tests.

**Run All Tests:**

```bash
cargo test
```

---

## Documentation

Generate and view the documentation using `cargo doc`.

```bash
cargo doc --open
```

This will build the documentation and open it in your default web browser.

---

## Contributing

Contributions are welcome! Please submit issues and pull requests on the [GitHub repository](https://github.com/yourusername/textcalc).

**To contribute:**

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes with clear messages.
4. Submit a pull request to the main branch.

---

## License

TextCalc is released under the [MIT License](LICENSE).

---

## Acknowledgments

TextCalc was developed using:

- [Rust Programming Language](https://www.rust-lang.org/)
- [Pest Parser](https://pest.rs/) for parsing expressions.
- [Clap](https://crates.io/crates/clap) for command-line argument parsing.
- [Reedline](https://crates.io/crates/reedline) for the interactive text-based interface.

---

## Contact

For any questions or feedback, please contact [Gavin Daly](mailto:gavin@gavdev.xyz).

---

Enjoy using TextCalc for all your calculation needs!
