use crate::lexer::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let lexer = Lexer::new(input);
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<u8, u8> {
        // Access tokens directly from the lexer
        let tokens = &self.lexer.tokens;

        println!("{:?}", tokens);
        // Implement your parsing logic using tokens

        Ok(1)
    }
}

