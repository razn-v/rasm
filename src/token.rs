/// List of available tokens
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Number,
    Label,
    Exclamation,
    Plus,
    Minus,
    OpenBracket,
    CloseBracket,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    Caret,
    Endline,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
}
