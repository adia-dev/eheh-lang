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
    DASTERISK,    // **
    DQUOTE,       // "
    EOF,          // \0
    EQ,           // ==
    EXPONENT,     // ^
    FORWARDSLASH, // /
    GT,           // >
    GTE,          // >=
    HASH,         // #
    IDENT,        // e.g: name
    ILLEGAL,      // ???
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
    STRING,     // e.g: "[...]"
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
    DEFER,
    ERRDEFER,
}

impl TokenType {
    pub fn to_literal(&self) -> String {
        match self {
            TokenType::AMPERSAND => "&".to_string(),
            TokenType::ASSIGN => "=".to_string(),
            TokenType::ASTERISK => "*".to_string(),
            TokenType::AND => "&&".to_string(),
            TokenType::AT => "@".to_string(),
            TokenType::BACKSLASH => "\\".to_string(),
            TokenType::BANG => "!".to_string(),
            TokenType::COLON => ":".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::COMMENT => "//".to_string(),
            TokenType::COMMENTBLOCK => "/*".to_string(),
            TokenType::DECR => "--".to_string(),
            TokenType::DOLLAR => "$".to_string(),
            TokenType::DOT => ".".to_string(),
            TokenType::DASTERISK => "**".to_string(),
            TokenType::DQUOTE => "\"".to_string(),
            TokenType::EOF => "<EOF>".to_string(),
            TokenType::EQ => "==".to_string(),
            TokenType::EXPONENT => "^".to_string(),
            TokenType::FORWARDSLASH => "/".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::GTE => ">=".to_string(),
            TokenType::HASH => "#".to_string(),
            TokenType::IDENT => "IDENT (e.g: first_name)".to_string(),
            TokenType::ILLEGAL => "???".to_string(),
            TokenType::INT => "e.g: 10".to_string(),
            TokenType::INCR => "++".to_string(),
            TokenType::LBRACE => "{".to_string(),
            TokenType::LBRACK => "[".to_string(),
            TokenType::LPAREN => "(".to_string(),
            TokenType::LSHIFT => "<<".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::LTE => "<=".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::NEQ => "!=".to_string(),
            TokenType::OR => "||".to_string(),
            TokenType::PERCENT => "%".to_string(),
            TokenType::PIPE => "|".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::QUESTION => "?".to_string(),
            TokenType::RANGE => "..".to_string(),
            TokenType::IRANGE => "..=".to_string(),
            TokenType::RBRACE => "}".to_string(),
            TokenType::RBRACK => "]".to_string(),
            TokenType::RPAREN => ")".to_string(),
            TokenType::RSHIFT => ">>".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::SCOPE => "::".to_string(),
            TokenType::SQUOTE => "'".to_string(),
            TokenType::STRING => "\"[...]\"".to_string(),
            TokenType::TILDE => "~".to_string(),
            TokenType::UNDERSCORE => "_".to_string(),
            TokenType::ARROW => "->".to_string(),
            TokenType::KEYWORD(kw) => kw.to_literal(),
        }
    }

    pub fn matching_delimiter(delimiter: TokenType) -> Option<TokenType> {
        let matching = match delimiter {
            TokenType::COMMENTBLOCK => TokenType::COMMENTBLOCK,
            TokenType::DQUOTE => TokenType::DQUOTE,
            TokenType::LBRACE => TokenType::RBRACE,
            TokenType::LBRACK => TokenType::RBRACK,
            TokenType::LPAREN => TokenType::RPAREN,
            TokenType::PIPE => TokenType::PIPE,
            TokenType::RBRACE => TokenType::LBRACE,
            TokenType::RBRACK => TokenType::LBRACK,
            TokenType::RPAREN => TokenType::LPAREN,
            TokenType::SQUOTE => TokenType::SQUOTE,
            _ => {
                return None;
            }
        };

        Some(matching)
    }
}

impl KeywordTokenType {
    pub fn to_literal(&self) -> String {
        match self {
            KeywordTokenType::FUN => "fn".to_string(),
            KeywordTokenType::LET => "let".to_string(),
            KeywordTokenType::CONST => "const".to_string(),
            KeywordTokenType::VAR => "var".to_string(),
            KeywordTokenType::TRUE => "true".to_string(),
            KeywordTokenType::FALSE => "false".to_string(),
            KeywordTokenType::IF => "if".to_string(),
            KeywordTokenType::ELSE => "else".to_string(),
            KeywordTokenType::END => "end".to_string(),
            KeywordTokenType::DO => "do".to_string(),
            KeywordTokenType::NULL => "null".to_string(),
            KeywordTokenType::RETURN => "return".to_string(),
            KeywordTokenType::DEFER => "defer".to_string(),
            KeywordTokenType::ERRDEFER => "errdefer".to_string(),
            kw => kw.to_string(),
        }
    }

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
            "defer" => Some(KeywordTokenType::DEFER),
            "errdefer" => Some(KeywordTokenType::ERRDEFER),
            _ => None,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for KeywordTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
