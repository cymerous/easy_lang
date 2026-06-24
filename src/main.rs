use std::fs::read_to_string;
pub mod tokenizer;
pub mod parser;

use crate::{parser::Parser, tokenizer::tokenize};

fn main() {
    let tokens = read_to_string("./main.el").unwrap();
    let mut parser = Parser::new(tokenize(&tokens));

    println!("{:?}", parser.parse());
}