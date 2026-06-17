use std::{fs::read_to_string, iter::Peekable, str::Chars};

use crate::Token::StringLiteral;

fn main() {
    let data = read_to_string("./main.el").unwrap();
    println!("{:?}", tokenize(&data));
}

#[derive(Debug)]
enum Token {
    Equal,
    While,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i32),
    For,
    Plus,
    Minus,
    Divide,
    Multiply,
    Variable,
    Switch,
}

fn tokenize(src: &str) -> Vec<Token> {
    let mut chars = src.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch == '=' {
            tokens.push(Token::Equal);
            chars.next();
            continue;
        }

        if ch == '"' {
            tokens.push(read_string(&mut chars))
        }

        if ch.is_ascii_digit() {
            //println!("{}", ch);
            tokens.push(read_number(&mut chars));
        }

        if ch.is_alphabetic() {
            let mut word = String::new();
            word.push(ch);
            chars.next();
            
            while let Some(next_char) = chars.next() {
                if next_char.is_alphanumeric() {
                    word.push(next_char);
                } else {
                    break;
                }
            }

            match word.as_str() {
                // reserved words
                "let" => tokens.push(Token::Variable),
                "while" => tokens.push(Token::While),
                "for" => tokens.push(Token::For),
                "switch" => tokens.push(Token::Switch),
                _ => tokens.push(Token::Identifier(word)),
            }
        }
    }

    tokens
}

fn read_string(chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut value = String::new();
    chars.next();

    while let Some(&next_char) = chars.peek() {
        if next_char == '"' {
            chars.next();
            break;
        }
        value.push(next_char);
        chars.next();
    }

    return Token::StringLiteral(value);
}

// TODO: add types for number to optimalize language but not for now.
/*enum NumberType {
    UnsignedInt64,
    UnsignedInt32,
    UnsignedInt16,
    UnsignedInt8,
    Int64,
    Int32,
    Int16,
    Int8
}*/

fn read_number(chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut value: i32 = 0;
    let mut sign = 1;

    while let Some(&next_char) = chars.peek() {
        //println!("{}", next_char);
        if next_char == '-' {
            sign = -1;
            chars.next();
            continue;
        }
        else if next_char.is_ascii_digit() {
            value = value * 10 + (next_char as i32 - '0' as i32);
            chars.next();
        } else {
            break;
        }
    }

    return Token::NumberLiteral(value * sign);
}