use self::token_type::TokenType;

pub mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
    pub line: usize,
    pub position: usize,
}

impl Token {
    pub fn new(t: TokenType, literal: String, line: usize, position: usize) -> Self {
        Self {
            t,
            literal,
            line,
            position,
        }
    }
}
