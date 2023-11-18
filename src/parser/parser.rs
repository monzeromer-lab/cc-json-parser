use crate::lexer::{lexer::Lexer, token::Token};


pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
}