#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Variable(String),
    BinaryOp {
        operator: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Definition {
        name: String,
        value: Expression,
    },
    Returnn(Expression),

    If {
        condition: Expression,
        then_block: Vec<Statement>,
        elif_blocks: Vec<(Expression, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
    },
    For {
        var: String,
        limit: i32,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
