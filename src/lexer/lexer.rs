use super::token::Kind;
use crate::lexer::token::Token;

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
    pub position: Position,
    pub tokens: Vec<Token>,
    pub current_token: Option<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: Position { line: 1, column: 1 },
            tokens: Vec::new(),
            current_token: None,
        };
        lexer.tokenize();
        lexer
    }

    pub fn set_input(&mut self, json_file_text: String) {
        self.input = json_file_text;
    }

    pub fn tokenize(&mut self) {
        while !self.is_at_end() {
            // self.position = self.position;
            self.scan_tokens();
        }
        self.add_token(Kind::EOFKind, "".to_string());
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => {
                self.add_token(Kind::LeftBraceKind, "{".to_string());
                self.advance();
            }
            '}' => {
                self.add_token(Kind::RightBraceKind, "}".to_string());
                self.advance();
            }
            ',' => {
                self.add_token(Kind::CommaKind, ",".to_string());
                self.advance();
            }
            ':' => {
                self.add_token(Kind::ColonKind, ":".to_string());
                self.advance();
            }
            ' ' | '\t' | '\r' => self.scan_whitespace(),
            '\n' => self.advance_line(),
            '"' => self.scan_string(),
            '0'..='9' => self.scan_number(),
            't' | 'f' => self.scan_boolean(),
            'n' => self.scan_null(),
            '\0' => self.current_token = None,
            _ => self.error(format!("Unexpected character: {}", c)),
        }
    }

    fn scan_string(&mut self) {
        let mut value = String::new();
        let start_pos = self.position;

        while let Some(c) = self.peek() {
            self.advance();
            match c {
                '"' => {
                    // End of the string
                    self.add_token(Kind::StringKind, value);
                    return;
                }
                '\\' => {
                    // Handle escape characters
                    if let Some(escaped_char) = self.peek() {
                        match escaped_char {
                            '"' | '\\' | '/' => value.push(escaped_char),
                            'b' => value.push('\u{0008}'), // Backspace
                            'f' => value.push('\u{000C}'), // Form feed
                            'n' => value.push('\n'),       // Newline
                            'r' => value.push('\r'),       // Carriage return
                            't' => value.push('\t'),       // Tab
                            'u' => {
                                // Handle Unicode escape sequence (e.g., \uFFFF)
                                // You need to implement the specific logic here
                                // and update the `value` accordingly.
                                // For simplicity, let's assume we append 'u' for now.
                                value.push('u');
                            }
                            _ => {
                                // Invalid escape sequence
                                self.error("Invalid escape sequence in string".to_owned());
                            }
                        }
                    } else {
                        // Unexpected end of input after '\'
                        self.error("Unexpected end of input after '\\' in string".to_owned());
                        return;
                    }
                }
                _ => {
                    // Regular character, add to the value
                    value.push(c);
                }
            }
        }

        self.error_at_current("Unterminated string");
    }

    fn scan_whitespace(&mut self) {
        let mut value = String::new();
        let start_pos = self.position;

        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        self.add_token(Kind::WhiteSpaceKind, value);
    }

    fn scan_number(&mut self) {
        let mut value = String::new();
        let start_pos = self.position;

        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if let Ok(parsed_value) = value.parse::<f64>() {
            self.add_token(Kind::FloatKind, value);
        } else {
            self.error_at(start_pos, &format!("Invalid number: {}", value));
        }
    }

    fn scan_boolean(&mut self) {
        let mut value = String::new();
        let start_pos = self.position;

        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match value.as_str() {
            "true" => self.add_token(Kind::BooleanKind, "true".to_string()),
            "false" => self.add_token(Kind::BooleanKind, "false".to_string()),
            _ => self.error_at(start_pos, &format!("Invalid boolean: {}", value)),
        }
    }

    fn scan_null(&mut self) {
        let mut value = String::new();
        let start_pos = self.position;

        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if value == "null" {
            self.add_token(Kind::NullKind, "null".to_string());
        } else {
            self.error_at(start_pos, &format!("Invalid null: {}", value));
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.chars().nth(self.position.column - 1)
    }

    fn advance_column(&mut self) {
        self.position.column += 1;
    }

    fn error_at(&self, position: Position, message: &str) {
        eprintln!(
            "Error at {}:{} - {}",
            position.line, position.column, message
        );
    }

    fn advance_line(&mut self) {
        self.position.line += 1;
        self.position.column = 1;
    }

    fn is_at_end(&self) -> bool {
        self.position.column > self.input.len() || self.current_token.is_none()
    }

    fn advance(&mut self) -> char {
        if let Some(c) = self.input.chars().nth(self.position.column - 1) {
            self.position.column += 1;
            c
        } else {
            '\0' // Return a sentinel value to indicate the end of the file
        }
    }

    fn add_token(&mut self, kind: Kind, value: String) {
        let token = Token {
            kind,
            value,
            line: self.position.line.clone(),
            column: self.position.column.clone(),
        };
        self.tokens.push(token);
    }

    fn error(&self, message: String) {
        eprintln!("Error: {}", message);
    }

    pub fn error_at_current(&self, message: &str) {
        let position = self.position;
        self.error_at(position, message);
    }
}
