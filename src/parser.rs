use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expressions {
    Number(i32),
    String(String),
    Bool(bool),
    VariableRef(String)
}

#[derive(Debug)]
pub enum Statements {
    VariableDeclaration {
        name: String,
        value: Expressions
    },

    WhileLoop {
        condition: Expressions,
        body: Vec<Statements>
    }
}

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable()
        }
    }

    pub fn parse(&mut self) -> Vec<Statements> {
        let mut program = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Variable => {
                    program.push(self.parse_variable_declaration());
                },
                _ => {
                    self.tokens.next();
                }
            }
        }

        program
    }

    fn parse_expression(&mut self) -> Expressions {
        match self.tokens.next() {
            Some(Token::BoolLiteral(value)) => Expressions::Bool(value),
            Some(Token::StringLiteral(value)) => Expressions::String(value),
            Some(Token::NumberLiteral(value)) => Expressions::Number(value),
            _ => panic!("Synax error: Invalid expression after '='")
        }
    }

    fn parse_variable_declaration(&mut self) -> Statements {
        self.tokens.next();
        let name = match self.tokens.next() {
            Some(Token::Identifier(value)) => value,
            _ => panic!("Syntax error: Name was expected after 'let'")
        };

        match self.tokens.next() {
            Some(Token::Equal) => {},
            _ => panic!("Syntax error: An '=' was expected after variable name.")
        };

        let value = self.parse_expression();
        Statements::VariableDeclaration { name, value }
    }
}