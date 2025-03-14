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
    println!("Welcome to the Rust Calculator!");
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

        match calculate_expression(input) {
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
            ' ' => {
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

    let mut result = parse_term(tokens, &mut index)?;

    while index < tokens.len() {
        match tokens[index] {
            Token::Plus => {
                index += 1;
                result += parse_term(tokens, &mut index)?;
            }
            Token::Minus => {
                index += 1;
                result -= parse_term(tokens, &mut index)?;
            }
            _ => break,
        }
    }

    Ok(result)
}

fn parse_term(tokens: &[Token], index: &mut usize) -> Result<f64, String> {
    let mut left = parse_factor(tokens, index)?;

    while *index < tokens.len() {
        match tokens[*index] {
            Token::Multiply => {
                *index += 1;
                let right = parse_factor(tokens, index)?;
                left = left * right;
            }
            Token::Divide => {
                *index += 1;
                let right = parse_factor(tokens, index)?;
                left = left / right;
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_factor(tokens: &[Token], index: &mut usize) -> Result<f64, String> {
    if *index >= tokens.len() {
        return Err("Unexpected end of input".to_string());
    }

    match tokens[*index] {
        Token::Number(n) => {
            *index += 1;
            Ok(n)
        }
        _ => Err("Invalid factor".to_string()),
    }
}

fn calculate_expression(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("Empty input".into());
    }
    println!("Tokens: {:?}", tokens);
    parse_expression(&tokens)
}
