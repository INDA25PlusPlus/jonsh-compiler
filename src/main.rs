use crate::backend::Generate;
use std::env;
use std::fs;

mod ast;
mod backend;
mod lexicon;
mod prassel;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Comma,
    LeftPar,
    RightPar,
    LeftBrace,
    RightBrace,
    Number(i32),
    Math(char),
    Operator(String),
    Terminator,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents = contents
        .replace("\n", "")
        .replace("\t", "")
        .replace(" ", "");
    let tokens = lexicon::tokenize(&contents);
    let mut program = prassel::Parser::new(tokens);
    let mut source_code = Generate::new(program.parse_program(), Vec::new(), 0);
    let rust_code = source_code.formatting();
    println!("{}", rust_code.as_str());
}
