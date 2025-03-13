use std::io::{self, Write};

#[derive(Debug)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn main() {
    println!("Weclome to the Rust Calculator!");
    println!("Enter 'quit' to exit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        match caculate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut number: String = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value = number.parse::<f64>().map_err(|_| "Invalid number")?;
                tokens.push(Token::Number(value));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(match c {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Multiply,
                    '/' => Token::Divide,
                    _ => unreachable!(),
                });
                chars.next();
            }
            _ => {
                return Err("Invalid character".to_string());
            }
        }
    }
    Ok(tokens)
}

fn parse_expression(tokens: &[Token]) -> Result<f64, String> {
    let mut index = 0;

    fn parse_term(tokens: &[Token], index: &mut usize) -> Result<f64, String> {
        // todo
        let mut left = parse_factor(tokens, index)?;

        Ok(left)
    }

    fn parse_factor(tokens: &[Token], index: &mut usize) -> Result<f64, String> {
        // todo
        Ok(0.0)
    }

    parse_term(tokens, &mut index)
}

fn caculate_expression(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Invalid input. Expected format: <number> <operator> <number>".to_string());
    }

    let num1: f64 = parts[0].parse().map_err(|_| "first number invalid")?;
    let operator = parts[1];
    let num2: f64 = parts[2].parse().map_err(|_| "second number invalid")?;

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}
