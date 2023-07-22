use std::fmt::Display;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACK,
    RBRACK,
    KEYWORD(KeywordTokenType),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum KeywordTokenType {
    FUN,
    LET,
    CONST,
}
impl KeywordTokenType {
    pub fn from_str(identifier: &str) -> Option<KeywordTokenType> {
        match identifier {
            "fn" => Some(KeywordTokenType::FUN),
            "let" => Some(KeywordTokenType::LET),
            "const" => Some(KeywordTokenType::CONST),
            _ => None
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
