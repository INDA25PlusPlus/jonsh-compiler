use crate::Token;
use crate::ast::{Expression, Statement};

pub struct Generate {
    statements: Vec<Statement>,
    pos: usize,
    identifiers: Vec<String>,
    space: i32,
}

impl Generate {
    pub fn new(statements: Vec<Statement>, identifiers: Vec<String>, space: i32) -> Self {
        Generate {
            statements,
            pos: 0,
            identifiers,
            space: space + 1,
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
                    string.push_str(format!("{}{}{}", &name, &operator, &expr).as_str());
                }
                string
            }
        }
    }

    fn generate_definition(&mut self) -> String {
        let mut strings = String::new();
        if let Statement::Definition { identifier, value } = self.get_statement() {
            if !self.identifiers.contains(&identifier.clone()) {
                strings.push_str("let mut ");
            }
            self.identifiers.push(identifier.clone());
            strings.push_str(format!("{}={};", identifier, self.from_expression(value)).as_str());
        }
        strings
    }

    fn generate_for(&mut self) -> String {
        let mut strings = String::new();
        if let Statement::For {
            index,
            expression,
            block,
        } = self.get_statement()
        {
            self.space += 1;
            let mut block_statements = Generate::new(*block, self.identifiers.clone(), self.space);
            let mut space = String::new();
            for _ in 0..(self.space - 1) {
                space.push_str("\t");
            }
            strings.push_str(
                format!(
                    "for {} in 1..{} {{\n{}{}}}\n",
                    index,
                    self.from_expression(expression),
                    block_statements.generate_code(self.space),
                    space
                )
                .as_str(),
            );
        }
        self.space += -1;
        strings
    }

    fn generate_if(&mut self) -> String {
        let mut strings = String::new();
        if let Statement::If { condition, block } = self.get_statement() {
            self.space += 1;
            let mut block_statements = Generate::new(*block, self.identifiers.clone(), self.space);
            let mut space = String::new();
            for _ in 0..(self.space - 1) {
                space.push_str("\t");
            }
            strings.push_str(
                format!(
                    "if {} {{\n{}{}}}\n",
                    self.from_expression(condition),
                    block_statements.generate_code(self.space),
                    space
                )
                .as_str(),
            );
        }
        self.space += -1;
        strings
    }

    fn generate_return(&mut self) -> String {
        let mut strings = String::new();
        if let Statement::Return(expression) = self.get_statement() {
            strings.push_str(
                format!("println!(\"{{}}\",{});", self.from_expression(expression)).as_str(),
            );
        }
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

    pub fn generate_code(&mut self, n: i32) -> String {
        let mut statements: Vec<String> = Vec::new();
        while self.pos < self.statements.len() {
            let mut space = String::new();
            for _ in 0..n {
                space.push_str("\t");
            }
            statements.push(space + self.generate_statement().as_str());
        }
        let statements: String = statements.join("");
        return statements;
    }

    pub fn formatting(&mut self) -> String {
        let mut code = self.generate_code(self.space);
        let mut space = String::new();
        for _ in 0..(self.space - 1) {
            space.push_str("\t");
        }
        code = format!(
            "{}fn main() {{\n{}\n{}}}",
            space,
            code.replace(";", ";\n"),
            space
        );
        return code;
    }
}

#[cfg(test)]
mod test {
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
        let mut code = Generate::new(ast, Vec::new(), 0);

        // let statement = code.generate_statement();
        // println!("Statement: {:?}", statement);
        println!("{}", code.formatting());
    }

    #[test]
    fn fib() {
        let mut n = 10;
        let mut a0 = 0;
        let mut a1 = 1;
        let mut an = 0;
        if n == 0 {
            println!("{}", 0);
        }
        if n == 1 {
            println!("{}", 1);
        }
        for i in 1..n {
            an = a1 + a0;
            a0 = a1;
            a1 = an;
        }
        println!("{}", an);
    }
}
