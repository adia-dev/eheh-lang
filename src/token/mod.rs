use self::token_type::TokenType;

pub mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(t: TokenType, literal: String) -> Self {
        Self { t, literal }
    }
}
