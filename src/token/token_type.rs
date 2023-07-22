use std::fmt::Display;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenType {
    AMPERSAND,
    ASSIGN,
    ASTERISK,
    AT,
    BACKSLASH,
    BANG,
    COLON,
    COMMA,
    DOLLAR,
    DOT,
    DQUOTE,
    EOF,
    FORWARDSLASH,
    GT,
    HASH,
    IDENT,
    ILLEGAL,
    INT,
    KEYWORD(KeywordTokenType),
    LBRACE,
    LBRACK,
    LPAREN,
    LT,
    MINUS,
    PERCENT,
    PIPE,
    PLUS,
    QUESTION,
    RBRACE,
    RBRACK,
    RPAREN,
    SEMICOLON,
    SQUOTE,
    TILDE,
    UNDERSCORE,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum KeywordTokenType {
    FUN,
    LET,
    CONST,
    TRUE,
    FALSE,
    IF,
    ELSE,
    END,
    DO,
}
impl KeywordTokenType {
    pub fn from_str(identifier: &str) -> Option<KeywordTokenType> {
        match identifier {
            "fn" => Some(KeywordTokenType::FUN),
            "let" => Some(KeywordTokenType::LET),
            "const" => Some(KeywordTokenType::CONST),
            "true" => Some(KeywordTokenType::TRUE),
            "false" => Some(KeywordTokenType::FALSE),
            "if" => Some(KeywordTokenType::IF),
            "else" => Some(KeywordTokenType::ELSE),
            "end" => Some(KeywordTokenType::END),
            "do" => Some(KeywordTokenType::DO),
            _ => None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
