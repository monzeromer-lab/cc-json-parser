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
}

pub struct TokenKind {
    pub kind: Kind,
    pub literal: String,
}