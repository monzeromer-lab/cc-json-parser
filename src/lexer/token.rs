#[derive(Debug, PartialEq)]
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
    SOFKind,
    EOFKind,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}