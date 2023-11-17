use crate::{structs::token::Token, lexer::Lexer};

pub struct Parser {
    lexer: Lexer,
    tokens: Vec<Token>,
}

