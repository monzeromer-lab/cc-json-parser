#[derive(Debug)]
pub enum Kind {
    IdentifierKind,
    IntegerKind,
    FloatKind,
    StringKind,
    NullKind,
    BooleanKind,
    WhiteSpaceKind,
    NewLineKind,
    LeftBraceKind,
    RightBraceKind,
    CommaKind,
    ColonKind,
    EOFKind,
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}