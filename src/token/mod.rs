use self::token_type::TokenType;

pub mod token_type;
#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(t: TokenType, literal: String, line: usize, column: usize) -> Self {
        Self {
            t,
            literal,
            line,
            column,
        }
    }

    pub fn get_location(&self) -> String {
        format!("{}:{}", self.line, self.column)
    }
}
