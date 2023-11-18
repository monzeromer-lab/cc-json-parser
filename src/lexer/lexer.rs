use crate::lexer::token::Token;

use super::token::TokenKind;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    // Add more error variants as needed
}

pub struct Lexer {
    input: String,
    position: Position,
    current_token: Token,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let position = Position { line: 1, column: 1 };
        let current_token = Token::EOF; // Assuming you have an EOF token in your Token enum
        Lexer {
            input,
            position,
            current_token,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        // Implement the logic to generate the next token from the input
        // You can update the position as you read characters
        // Update the current_token field accordingly
        // Return Ok(Token) when a valid token is found, or Err(LexerError) for errors

        // Placeholder logic, replace with actual implementation
        if let Some(ch) = self.input.chars().next() {
            // Implement your tokenization logic here
            let token = match ch {
                // Handle different character cases
                _ => return Err(LexerError::UnexpectedCharacter(ch)),
            };
            // Update position and consume the character
            self.position.column += 1;
            self.input = self.input[1..].to_string();
            Ok(token)
        } else {
            Ok(TokenKind::EOFKind) // Placeholder for end of input, replace with actual logic
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    pub fn peek(&self) -> Option<&Token> {
        Some(&self.current_token)
    }

    pub fn advance(&mut self) -> Result<Token, LexerError> {
        let token = self.next_token()?;
        self.current_token = token;
        Ok(token)
    }

    pub fn error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }

    pub fn error_at(&self, message: &str, position: Position) {
        eprintln!("Error at {}:{} - {}", position.line, position.column, message);
    }

    pub fn error_at_current(&self, message: &str) {
        let position = self.position();
        eprintln!("Error at {}:{} - {}", position.line, position.column, message);
    }
}

// Additional functions and implementations can be added as needed
