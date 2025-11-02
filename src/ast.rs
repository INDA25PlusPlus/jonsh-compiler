use crate::Token;

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
pub enum Statement {
    Definition {
        name: String,
        value: Expression,
    },
    Return(Expression),
    If {
        condition: Expression,
        block: Box<Vec<Statement>>,
    },
    For {
        name: String,
        expression: Expression,
        block: Box<Vec<Statement>>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
