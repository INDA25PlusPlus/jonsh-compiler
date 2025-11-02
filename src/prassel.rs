use crate::Token;
use crate::ast::{Expression, Program, Statement};
use crate::lexicon::tokenize;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn get_token(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos = self.pos + 1;
        return token;
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
        if let Token::Identifier(name) = self.get_token() {
            if self.current_token() == Token::Operator("=".to_string()) {
                self.pos = self.pos + 1;
                let expr = self.parse_expression();
                return Statement::Definition {
                    name: name,
                    value: expr,
                };
            } else {
                panic!("Stoopid");
            }
        } else {
            panic!(
                "Failed to get Identifier token, processing current token: {:?}",
                self.current_token()
            );
        }
    }

    fn parse_keyword(&mut self) -> Statement {
        if let Token::Keyword(val) = self.get_token() {
            if self.get_token() != Token::LeftPar {
                panic!("expected Left Parenthesis")
            }
            match val.as_str() {
                "return" => {
                    let expression = self.parse_expression();
                    if self.get_token() != Token::RightPar {
                        panic!(
                            "Non Closed paranthesis, Current Token: {:?}",
                            self.current_token()
                        )
                    }
                    return Statement::Return(expression);
                }
                "if" => {
                    let condition = self.parse_expression();
                    if self.get_token() != Token::RightPar {
                        panic!(
                            "Syntax failed on RightPar, Current Token: {:?}",
                            self.current_token()
                        )
                    }
                    if self.get_token() != Token::LeftBrace {
                        panic!(
                            "Syntax failed on LeftBrace, Current Token: {:?}",
                            self.current_token()
                        );
                    }
                    let block = Box::new(self.parse_program());
                    if self.get_token() == Token::RightBrace {
                        return Statement::If { condition, block };
                    } else {
                        panic!("No closing bracket at {}", self.pos)
                    }
                }
                "for" => {
                    if let Token::Identifier(name) = self.get_token() {
                        if self.get_token() != Token::Comma {
                            panic!(
                                "Syntax failed on Comma , Current Token: {:?}",
                                self.current_token()
                            )
                        }
                        let expression = self.parse_expression();
                        if self.get_token() != Token::RightPar {
                            panic!(
                                "Syntax failed on RightPar, Current Token: {:?}",
                                self.current_token()
                            );
                        }
                        if self.get_token() != Token::LeftBrace {
                            panic!(
                                "Syntax failed on LeftBrace, Current Token: {:?}",
                                self.current_token()
                            );
                        }
                        let block = Box::new(self.parse_program());
                        if self.get_token() == Token::RightBrace {
                            return Statement::For {
                                name,
                                expression,
                                block,
                            };
                        } else {
                            panic!("No closing bracket at {}", self.pos)
                        }
                    } else {
                        panic!(
                            "Syntax Failed on Identifier, Current Token: {:?}",
                            self.current_token()
                        )
                    }
                }
                _ => panic!("No such keyword as {val}"),
            }
        } else {
            panic!("Syntax failed on keyword function")
        }
    }

    fn parse_expression(&mut self) -> Expression {
        let left = self.get_token();

        if let Token::Math(sign) = self.current_token() {
            self.pos = self.pos + 1;
            let right = Box::new(self.parse_expression());
            return Expression::BinaryOp {
                operator: sign.to_string(),
                left: left,
                right: right,
            };
        }
        // else if let Token::Terminator = self.current_token() {
        //     match left {
        //         Token::Identifier(name) => return Expression::Variable(name),
        //         Token::Number(value) => return Expression::Number(value),
        //         _ => panic!("WHAT THE FUCK"),
        //     }
        // } else if let Token::Comma = self.current_token() {
        //     match left {
        //         Token::Identifier(name) => return Expression::Variable(name),
        //         Token::Number(value) => return Expression::Number(value),
        //         _ => panic!("WHAT THE FUCK PART 2"),
        //     }
        // }
        else if let Token::Operator(sign) = self.current_token() {
            self.pos = self.pos + 1;
            let right = Box::new(self.parse_expression());
            return Expression::BinaryOp {
                operator: sign,
                left: left,
                right: right,
            };
        } else {
            // self.pos = self.pos + 1;
            match left {
                Token::Identifier(name) => return Expression::Identifier(name),
                Token::Number(value) => return Expression::Number(value),
                _ => panic!("WHAT THE FUCK"),
            }
        }
    }

    fn parse_statement(&mut self) -> Statement {
        let statement: Statement;
        match self.current_token() {
            Token::Keyword(_) => statement = self.parse_keyword(),
            Token::Identifier(_) => statement = self.parse_definition(),
            _ => panic!(
                "PARSE STATEMENT FAILED ATT TOKEN: {:?}",
                self.current_token()
            ),
        }
        if self.get_token() == Token::Terminator {
            return statement;
        } else {
            panic!("Expected ';', Current Token: {:?}", self.current_token())
        }
    }
}

#[test]
fn test() {
    let program = "a=2;b=2;c=a+b+2+c+2;".to_string();
    let tokens = tokenize(&program);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    println!("AST:\n{:#?}", ast);
}
