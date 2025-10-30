pub fn tokenize(input: &String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    for c in input.chars() {
        if [
            "+", "-", "*", "/", "=", "<", ">", "!", "(", ")", "{", "}", ",", ";",
        ]
        .contains(&c.to_string().as_str())
        {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else {
            current_token.push(c);
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    for token in &mut tokens {
        if ["return", "if", "else", "elif", "while", "for"].contains(&token.as_str()) {
            *token = format!("Keyword({})", token);
        } else if token.parse::<i32>().is_ok() {
            *token = format!("Number({})", token);
        } else if ["+", "-", "*", "/", "=", "<", ">", "<=", ">=", "!"].contains(&token.as_str()) {
            *token = format!("Operator({})", token);
        } else if ["(", ")", "{", "}", ","].contains(&token.as_str()) {
            *token = format!("Symbol({})", token);
        } else if token == ";" {
            *token = "Terminator".to_string();
        } else {
            *token = format!("Identifier({})", token);
        }
    }
    tokens
}
