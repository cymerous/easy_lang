use std::fs::read_to_string;
mod tokenizer;

fn main() {
    let data = read_to_string("./main.el").unwrap();
    println!("{:?}", tokenizer::tokenize(&data));
}