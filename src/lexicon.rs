use crate::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut iter: Peekable<Chars> = input.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            // punctuation that we keep as Symbol/Terminator
            '(' | ')' | '{' | '}' | ',' => {
                if !current_token.is_empty() {
                    tokens.push(Token::Identifier(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::Symbol(c));
            }
            ';' => {
                if !current_token.is_empty() {
                    tokens.push(Token::Identifier(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::Terminator);
            }
            // operators: try to form two-char operators first
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' => {
                if !current_token.is_empty() {
                    tokens.push(Token::Identifier(current_token.clone()));
                    current_token.clear();
                }

                // try two-char operators: ==, !=, <=, >=
                if let Some(&next) = iter.peek() {
                    let two = format!("{}{}", c, next);
                    match two.as_str() {
                        "==" | "!=" | "<=" | ">=" => {
                            // consume next
                            iter.next();
                            tokens.push(Token::Operator(two));
                            continue;
                        }
                        _ => {}
                    }
                }

                // single-char operator -> Operator(String)
                tokens.push(Token::Operator(c.to_string()));
            }
            ch if ch.is_whitespace() => {
                if !current_token.is_empty() {
                    tokens.push(Token::Identifier(current_token.clone()));
                    current_token.clear();
                }
            }
            other => {
                current_token.push(other);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(Token::Identifier(current_token));
    }

    // convert identifiers to Keyword or Number where applicable
    for token in &mut tokens {
        if let Token::Identifier(s) = token {
            match s.as_str() {
                "return" | "if" | "while" | "for" | "print" => {
                    *token = Token::Keyword(s.clone());
                }
                _ => {
                    if let Ok(num) = s.parse::<i32>() {
                        *token = Token::Number(num);
                    }
                }
            }
        }
    }

    tokens
}
