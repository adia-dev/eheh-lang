use std::fmt::Display;

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenType {
    AMPERSAND,    // &
    ASSIGN,       // =
    ASTERISK,     // *
    AND,          // &&
    AT,           // @
    BACKSLASH,    // \
    BANG,         // !
    COLON,        // :
    COMMA,        // ,
    COMMENT,      // //
    COMMENTBLOCK, // /*
    DECR,         // --
    DOLLAR,       // $
    DOT,          // .
    DQUOTE,       // "
    EOF,          // \0
    EQ,           // ==
    EXPONENT,     // ^
    FORWARDSLASH, // /
    GT,           // >
    GTE,          // >=
    HASH,         // #
    IDENT,        // e.g: name
    ILLEGAL,      // unsupported tokens
    INT,          // e.g: 10
    INCR,         // ++
    KEYWORD(KeywordTokenType),
    LBRACE,     // {
    LBRACK,     // [
    LPAREN,     // (
    LSHIFT,     // <<
    LT,         // <
    LTE,        // <=
    MINUS,      // -
    NEQ,        // !=
    OR,         // ||
    PERCENT,    // %
    PIPE,       // |
    PLUS,       // +
    QUESTION,   // ?
    RANGE,      // ..
    IRANGE,     // ..=
    RBRACE,     // }
    RBRACK,     // ]
    RPAREN,     // )
    RSHIFT,     // >>
    SEMICOLON,  // ;
    SCOPE,      // ::
    SQUOTE,     // '
    STRING,     // e.g: "Abdoulaye"
    TILDE,      // ~
    UNDERSCORE, // _
    ARROW,      // ->
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum KeywordTokenType {
    FUN,
    LET,
    CONST,
    VAR,
    TRUE,
    FALSE,
    IF,
    ELSE,
    END,
    DO,
    NULL,
    RETURN,
}
impl KeywordTokenType {
    pub fn from_str(identifier: &str) -> Option<KeywordTokenType> {
        match identifier {
            "fn" => Some(KeywordTokenType::FUN),
            "let" => Some(KeywordTokenType::LET),
            "const" => Some(KeywordTokenType::CONST),
            "var" => Some(KeywordTokenType::VAR),
            "true" => Some(KeywordTokenType::TRUE),
            "false" => Some(KeywordTokenType::FALSE),
            "if" => Some(KeywordTokenType::IF),
            "else" => Some(KeywordTokenType::ELSE),
            "end" => Some(KeywordTokenType::END),
            "do" => Some(KeywordTokenType::DO),
            "null" => Some(KeywordTokenType::NULL),
            "return" => Some(KeywordTokenType::RETURN),
            _ => None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
