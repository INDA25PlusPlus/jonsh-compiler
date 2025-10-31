// ...existing code...
use crate::Token;
use crate::ast::{Expression, Program, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
                // advance one token when nothing was parsed to avoid infinite loop
                self.position += 1;
            }
        }

        Program { statements }
    }

    fn current_statement(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_statement() {
            Some(Token::Identifier(_)) => self.parse_definition(),
            Some(Token::Keyword(_)) => self.parse_compare(),
            _ => None,
        }
    }

    fn parse_definition(&mut self) -> Option<Statement> {
        if let Some(Token::Identifier(name)) = self.current_statement().cloned() {
            self.position += 1;
            match self.current_statement() {
                Some(Token::Symbol('=')) => {
                    self.position += 1;
                }
                Some(Token::Operator(op)) if op == "=" => {
                    self.position += 1;
                }
                _ => return None,
            }
            if let Some(value) = self.parse_expression() {
                // consume optional terminator
                if matches!(self.current_statement(), Some(Token::Terminator)) {
                    self.position += 1;
                }
                println!("Parsing definition for {}", name);
                return Some(Statement::Definition { name, value });
            }
        }
        None
    }

    // precedence helpers
    fn precedence(op: &str) -> i32 {
        match op {
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 5,
            "+" | "-" => 10,
            "*" | "/" => 20,
            _ => 0,
        }
    }
    fn is_right_associative(op: &str) -> bool {
        matches!(op, "^")
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match self.current_statement().cloned() {
            Some(Token::Number(n)) => {
                self.position += 1;
                Some(Expression::Number(n))
            }
            Some(Token::Identifier(name)) => {
                self.position += 1;
                Some(Expression::Variable(name))
            }
            Some(Token::Symbol('(')) => {
                self.position += 1; // consume '('
                let expr = self.parse_expression_prec(0);
                if matches!(self.current_statement(), Some(Token::Symbol(')'))) {
                    self.position += 1; // consume ')'
                }
                expr
            }
            _ => None,
        }
    }

    fn parse_expression_prec(&mut self, min_prec: i32) -> Option<Expression> {
        let mut left = self.parse_primary()?;

        loop {
            // accept Operator(String) tokens only (lexicon now makes operators Operator)
            let op = match self.current_statement() {
                Some(Token::Operator(s)) => s.clone(),
                _ => break,
            };

            let prec = Self::precedence(&op);
            if prec < min_prec {
                break;
            }

            // consume operator token
            self.position += 1;

            let next_min = if Self::is_right_associative(&op) {
                prec
            } else {
                prec + 1
            };

            let right = match self.parse_expression_prec(next_min) {
                Some(r) => r,
                None => return None,
            };

            left = Expression::BinaryOp {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_expression_prec(0)
    }

    fn parse_compare(&mut self) -> Option<Statement> {
        println!("Parsing comparison");
        if let Some(Token::Keyword(keyword)) = self.current_statement().cloned() {
            if keyword == "if" {
                println!("Found if keyword");
                self.position += 1;
                if matches!(self.current_statement(), Some(Token::Symbol('('))) {
                    self.position += 1;
                    println!("Parsing if statement");
                    if let Some(condition) = self.parse_expression() {
                        println!("Parsed condition: {:?}", condition);
                        // expect closing ')'
                        if matches!(self.current_statement(), Some(Token::Symbol(')'))) {
                            self.position += 1;
                            if matches!(self.current_statement(), Some(Token::Symbol('{'))) {
                                self.position += 1;
                                let mut then_block = Vec::new();
                                while let Some(token) = self.current_statement() {
                                    println!("Found token in then block: {:?}", token);
                                    if *token == Token::Symbol('}') {
                                        println!("End of then block");
                                        break;
                                    }
                                    if let Some(statement) = self.parse_statement() {
                                        then_block.push(statement);
                                    } else {
                                        // advance if nothing parsed
                                        self.position += 1;
                                    }
                                }
                                if matches!(self.current_statement(), Some(Token::Symbol('}'))) {
                                    self.position += 1;
                                    // consume optional terminator after block
                                    if matches!(self.current_statement(), Some(Token::Terminator)) {
                                        self.position += 1;
                                    }
                                    return Some(Statement::If {
                                        condition,
                                        then_block,
                                        elif_blocks: Vec::new(),
                                        else_block: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
// ...existing code...
