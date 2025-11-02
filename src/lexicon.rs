use crate::Token;

pub fn tokenize(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();

    for c in input.chars() {
        if !current_token.is_empty() {
            //IF current token isn't empty
            if ["for", "if", "return"].contains(&current_token.as_str()) {
                if c == '(' {
                    tokens.push(Token::Keyword(current_token.clone()));
                    current_token.clear();
                } else {
                    current_token.push(c);
                }
            } else if current_token.chars().all(|s| s.is_alphanumeric()) {
                if !c.is_alphanumeric() {
                    if current_token.parse::<i32>().is_ok() {
                        tokens.push(Token::Number(current_token.clone().parse().unwrap()));
                        current_token.clear();
                    } else if current_token.parse::<String>().is_ok() {
                        tokens.push(Token::Identifier(current_token.clone()));
                        current_token.clear();
                    }
                } else {
                    current_token.push(c);
                }
            } else if ["==", "!="].contains(&current_token.clone().as_str()) {
                tokens.push(Token::Operator(current_token.clone()));
                current_token.clear();
            } else if ["=", "!"].contains(&current_token.clone().as_str()) {
                if c == '=' {
                    current_token.push(c);
                } else {
                    tokens.push(Token::Operator(current_token.clone()));
                    current_token.clear();
                }
            }
        }
        if current_token.is_empty() {
            //IF current token is empty
            if c.is_alphanumeric() {
                current_token.push(c);
            } else {
                match c {
                    '(' => tokens.push(Token::LeftPar),
                    ')' => tokens.push(Token::RightPar),
                    '{' => tokens.push(Token::LeftBrace),
                    '}' => tokens.push(Token::RightBrace),
                    '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c.to_string())),
                    '<' | '>' => tokens.push(Token::Operator(c.to_string())),
                    '=' | '!' => current_token.push(c),
                    ',' => tokens.push(Token::Comma),
                    ';' => tokens.push(Token::Terminator),
                    _ => {}
                }
            }
        }
    }
    return tokens;
}

#[test]
fn test_tokenize() {
    let string = "n=10;
a0=0;
a1=1;
an=0;
if(n==0){return(0);};
if(n==1){return(1);};
for(i,n){an=a1+a0;a0=a1;a1=an};
return(an);"
        .to_string();
    println!("{:?}", tokenize(&string));
}
