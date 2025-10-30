use std::env;
use std::fs;

mod lexicon;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut contents =
        fs::read_to_string(file_path).expect("Shoyld have been able to read the file");
    contents = contents.replace("\n", "").replace("\t", "");
    let contents: String = contents.split_whitespace().collect();
    let tokens: String = lexicon::tokenize(&contents).join(", ");

    println!("{:?}", tokens);
}
