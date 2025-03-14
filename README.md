# Rust Calculator CLI

A simple command-line calculator application written in Rust that supports basic arithmetic operations.

## Features

- Interactive command-line interface
- Supports basic arithmetic operations:
  - Addition (+)
  - Subtraction (-)
  - Multiplication (\*)
  - Division (/)
- Error handling for invalid inputs
- Follows order of operations (multiplication and division before addition and subtraction)

## Installation

### Prerequisites

- Rust and Cargo (install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

### Building from Source

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rust-calculator-cli.git
   cd rust-calculator-cli
   ```

2. Build the project:

   ```
   cargo build --release
   ```

3. Run the calculator:

   ```
   cargo run --release
   ```

## Usage

After starting the application, you'll see a prompt where you can enter mathematical expressions:

```
Welcome to the Rust Calculator!
Enter 'quit' to exit.
> 2 + 3
Tokens: [Number(2.0), Plus, Number(3.0)]
Result: 5
> 10 * 5 - 8
Tokens: [Number(10.0), Multiply, Number(5.0), Minus, Number(8.0)]
Result: 42
> quit
```

### Supported Operations

- Addition: `a + b`
- Subtraction: `a - b`
- Multiplication: `a * b`
- Division: `a / b`

The calculator follows the standard order of operations (multiplication and division before addition and subtraction).

## Project Structure

- `src/main.rs`: Contains all the calculator logic including tokenization, parsing, and evaluation

## Development

### Building and Testing

```
# Build the project
cargo build

# Run the tests
cargo test

# Run the application
cargo run
```

## License

This project is open source and available under the [MIT License](LICENSE).
