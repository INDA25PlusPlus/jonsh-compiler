use crate::Token;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    BinaryOp {
        operator: String,
        left: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement {
    Definition {
        identifier: String,
        value: Expression,
    },
    Return(Expression),
    If {
        condition: Expression,
        block: Box<Vec<Statement>>,
    },
    For {
        index: String,
        expression: Expression,
        block: Box<Vec<Statement>>,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
