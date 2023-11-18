pub enum Kind {
    IdentifierKind,
    IntegerKind,
    FloatKind,
    StringKind,
    BooleanKind,
    WhiteSpaceKind,
    NewLineKind,
    LeftBraceKind,
    RightBraceKind,
    CommaKind,
    ColonKind,
    EOFKind,
}

pub struct TokenKind {
    pub kind: Kind,
    pub literal: String,
}

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}