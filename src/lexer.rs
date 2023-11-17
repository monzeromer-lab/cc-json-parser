use crate::structs::token::Token;

pub struct Position {
    line: usize,
    column: usize,
}

pub struct Lexer {
    input: String,
    position: Position,
    current_token: Token,
}

impl Lexer {

    /// generate tokens from the input string
    pub fn new(input: String) -> Vec<Token> {
        vec![]
    }

    /// move to the next token
    pub fn next_token(&mut self) -> Option<Token> {
        None
    }

    /// get the current position
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Update the position of the lexer
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Update the lexer input 
    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    /// Get the current token
    pub fn peek(&self) -> Option<&Token> {
        Some(&self.current_token)
    }

    /// this function will update the current token with the next one and return it
    pub fn advance(&mut self) -> Option<Token> {
        None
    }

    /// this will throw an error and close kill the process 
    pub fn error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }

    /// this should create a useful error message with the current position of the unknown token
    pub fn error_at(&self, message: &str, position: Position) {
        println!("Error: {}", message);
    }


    pub fn error_at_current(&self, message: &str) {
        println!("Error: {}", message);
    }

}