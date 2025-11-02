use crate::Token;
use crate::ast::{Expression, Statement};

pub struct Generate {
    statements: Vec<Statement>,
    pos: usize,
    identifiers: Vec<String>,
}

impl Generate {
    pub fn new(statements: Vec<Statement>, identifiers: Vec<String>) -> Self {
        Generate {
            statements,
            pos: 0,
            identifiers,
        }
    }

    fn get_statement(&mut self) -> Statement {
        let statement = self.statements[self.pos].clone();
        self.pos += 1;
        statement
    }

    fn current_statement(&self) -> Statement {
        self.statements[self.pos].clone()
    }

    fn from_expression(&self, expression: Expression) -> String {
        let mut string = String::new();
        match expression {
            Expression::Number(value) => return value.to_string(),
            Expression::Identifier(identifier) => return identifier,
            Expression::BinaryOp {
                operator,
                left,
                right,
            } => {
                if let Token::Identifier(name) = left {
                    let expr = self.from_expression(*right);
                    string.push_str(&name);
                    string.push_str(&operator);
                    string.push_str(&expr);
                    return string;
                }
                panic!("Error")
            }
        }
    }

    fn generate_definition(&mut self) -> String {
        let mut strings: Vec<String> = Vec::new();
        if let Statement::Definition { identifier, value } = self.get_statement() {
            if !self.identifiers.contains(&identifier.clone()) {
                strings.push("let mut ".to_string());
            }
            strings.push(identifier.clone());
            self.identifiers.push(identifier);
            strings.push("=".to_string());
            strings.push(self.from_expression(value));
            strings.push(";".to_string());
        }
        let strings: String = strings.join("");
        return strings;
    }

    fn generate_for(&mut self) -> String {
        let mut strings: Vec<String> = vec!["for ".to_string()];
        if let Statement::For {
            index,
            expression,
            block,
        } = self.get_statement()
        {
            strings.push(index);
            strings.push(" in 1..".to_string());
            strings.push(self.from_expression(expression));
            strings.push(" { \n\t".to_string());
            let mut block_statements = Generate::new(*block, self.identifiers.clone());
            strings.push(
                block_statements
                    .generate_code()
                    .split("\n")
                    .collect::<Vec<_>>()
                    .join("\n\t"),
            );
            strings.push("\n}".to_string());
        }
        let strings: String = strings.join("");
        strings
    }
    fn generate_if(&mut self) -> String {
        let mut strings: Vec<String> = vec!["if ".to_string()];
        if let Statement::If { condition, block } = self.get_statement() {
            strings.push(self.from_expression(condition));
            strings.push(" { \n\t".to_string());
            let mut block_statements = Generate::new(*block, self.identifiers.clone());
            strings.push(block_statements.generate_code());
            strings.push("\n}".to_string());
        }
        let strings: String = strings.join("");
        strings
    }
    fn generate_return(&mut self) -> String {
        let mut strings: Vec<String> = vec!["return ".to_string()];
        if let Statement::Return(expression) = self.get_statement() {
            strings.push(self.from_expression(expression));
            strings.push(";".to_string());
        }
        let strings: String = strings.join("");
        strings
    }

    pub fn generate_statement(&mut self) -> String {
        let mut string = String::new();
        match self.current_statement() {
            Statement::Definition { .. } => string.push_str(&self.generate_definition()),
            Statement::For { .. } => string.push_str(&self.generate_for()),
            Statement::If { .. } => string.push_str(&self.generate_if()),
            Statement::Return(_) => string.push_str(&self.generate_return()),
        }
        return string;
    }

    pub fn generate_code(&mut self) -> String {
        let mut statements: Vec<String> = Vec::new();
        while self.pos < self.statements.len() {
            statements.push(self.generate_statement());
        }
        let statements: String = statements.join("\n");
        return statements;
    }
}

#[cfg(test)]
mod test {
    use crate::Token;
    use crate::ast::Expression;
    use crate::backend::Generate;
    use crate::lexicon::tokenize;
    use crate::prassel::Parser;

    #[test]
    fn test() {
        let program = "n=10;
a0=0;
a1=1;
an=0;
if(n==0){return(0);};
if(n==1){return(1);};
for(i,n){an=a1+a0;a0=a1;a1=an;};
return(an);"
            .to_string();
        let tokens = tokenize(&program);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();
        let mut code = Generate::new(ast, Vec::new());
        // let statement = code.generate_statement();
        // println!("Statement: {:?}", statement);
        println!("{}", code.generate_code());
    }

    #[test]
    fn fib() {
        let mut n = 10;
        let mut a0 = 0;
        let mut a1 = 1;
        let mut an = 0;
        if n == 0 {
            println!("0");
        }
        if n == 1 {
            println!("1");
        }
        for i in 1..n {
            an = a1 + a0;
            a0 = a1;
            a1 = an;
        }
        println!("{}", an);
    }
}
