use self::token_type::TokenType;

pub mod token_type;
#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl Token {
    pub fn new(
        t: TokenType,
        literal: String,
        line: usize,
        column: usize,
        file: Option<String>,
    ) -> Self {
        Self {
            t,
            literal,
            line,
            column,
            file,
        }
    }

    pub fn get_location(&self) -> String {
        if let Some(file) = &self.file {
            format!("{}:{}:{}", file, self.line, self.column)
        } else {
            format!("{}:{}", self.line, self.column)
        }
    }
}
