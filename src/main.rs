use std::env;
use std::fs;

mod ast;
mod lexicon;
mod prassel;
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Symbol(char),
    Number(i32),
    Operator(String),
    Terminator,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut contents =
        fs::read_to_string(file_path).expect("Shoyld have been able to read the file");
    contents = contents.replace("\n", "").replace("\t", "");
    // let contents: String = contents.split_whitespace().collect();
    let tokens = lexicon::tokenize(&contents);
    println!("{:#?}", tokens);
    let mut parser = prassel::Parser::new(tokens);
    let program = parser.parse();
    println!("{:#?}", program);
}
