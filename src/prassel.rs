use crate::Token;
use crate::ast::{Expression, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn get_token(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos = self.pos + 1;
        return token;
    }

    fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.pos < self.tokens.len() {
            if self.current_token() == Token::RightBrace {
                break;
            }
            statements.push(self.parse_statement());
        }
        return statements;
    }

    fn parse_definition(&mut self) -> Statement {
        if let Token::Identifier(identifier) = self.get_token() {
            if self.current_token() == Token::Operator("=".to_string()) {
                self.pos = self.pos + 1;
                let expr = self.parse_expression();
                return Statement::Definition {
                    identifier,
                    value: expr,
                };
            }
            panic!("Expected '=' at pos: {}", self.pos);
        }
        panic!("Expected identifier at pos: {}", self.pos);
    }

    fn parse_keyword(&mut self) -> Statement {
        if let Token::Keyword(val) = self.get_token() {
            let expression: Expression;
            if self.get_token() != Token::LeftPar {
                panic!("Expected '(' at pos: {}", self.pos)
            };
            match val.as_str() {
                "return" => {
                    expression = self.parse_expression();
                }
                "if" => {
                    expression = self.parse_expression();
                }
                "for" => {
                    if let Token::Identifier(index) = self.get_token() {
                        if self.get_token() != Token::Comma {
                            panic!("Expected ',' at pos: {}", self.pos)
                        };
                        expression = self.parse_expression();
                        if self.get_token() != Token::RightPar {
                            panic!("Expected ')' at pos: {}", self.pos)
                        };
                        if self.get_token() != Token::LeftBrace {
                            panic!("Expected '{{' at pos {}", self.pos)
                        };
                        let block = Box::new(self.parse_program());
                        if self.get_token() != Token::RightBrace {
                            panic!("Expected '}}' at pos {}", self.pos)
                        }
                        return Statement::For {
                            index,
                            expression,
                            block,
                        };
                    }
                    panic!("Expected identifier at pos: {}", self.pos);
                }
                _ => panic!("Shouldn't happen"),
            }
            if self.get_token() != Token::RightPar {
                panic!("Expected ')' at pos: {}", self.pos)
            }
            if val.as_str() == "return" {
                return Statement::Return(expression);
            }
            if self.get_token() != Token::LeftBrace {
                panic!("Expected '{{' at pos {}", self.pos)
            }
            let block = Box::new(self.parse_program());
            if self.get_token() != Token::RightBrace {
                panic!("Expected '}}' at pos {}", self.pos)
            }
            if val.as_str() == "if" {
                return Statement::If {
                    condition: expression,
                    block,
                };
            }
            panic!("Shouldn't happen")
        }
        panic!("Shouldn't happen")
    }

    fn parse_expression(&mut self) -> Expression {
        let left = self.get_token();
        if let Token::Operator(operator) = self.current_token() {
            self.pos += 1;
            return Expression::BinaryOp {
                operator,
                left,
                right: Box::new(self.parse_expression()),
            };
        }
        match left {
            Token::Identifier(identifier) => return Expression::Identifier(identifier),
            Token::Number(value) => return Expression::Number(value),
            _ => panic!("Expected expression at pos: {}", self.pos),
        }
    }

    fn parse_statement(&mut self) -> Statement {
        let statement: Statement;
        match self.current_token() {
            Token::Keyword(_) => statement = self.parse_keyword(),
            Token::Identifier(_) => statement = self.parse_definition(),
            _ => panic!("expected keyword or identifier at pos: {}", self.pos),
        }
        if self.get_token() == Token::Terminator {
            return statement;
        }
        panic!("Expected ';', at pos: {}", self.pos)
    }
}

#[cfg(test)]
mod test {
    use crate::lexicon::tokenize;
    use crate::prassel::Parser;
    #[test]
    fn test() {
        let program = "a=1+1;".to_string();
        let tokens = tokenize(&program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();
        println!("AST:\n{:#?}", ast);
    }
}
