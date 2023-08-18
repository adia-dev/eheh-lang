use crate::token::{
    token_type::{KeywordTokenType, TokenType},
    Token,
};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: Vec<char>,
    pub raw_input: String,
    pub line: usize,
    pub last_new_line: usize,
    pub position: usize,
    pub next_position: usize,
    pub c: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut new_lexer = Self {
            input: input.chars().into_iter().collect(),
            raw_input: input.to_owned(),
            position: 0,
            next_position: 0,
            c: '\0',
            line: 1,
            last_new_line: 0,
        };

        new_lexer.advance();

        new_lexer
    }

    pub fn get_line(&self, n: usize) -> Option<String> {
        let splitted_code = self.raw_input.split('\n').collect::<Vec<&str>>();

        if n - 1 >= splitted_code.len() {
            None
        } else {
            Some(splitted_code[n - 1].to_string())
        }
    }

    pub fn scan(&mut self) -> Token {
        self.eat_whitespace();

        let mut token = Token::new(
            TokenType::ILLEGAL,
            self.c.to_string(),
            self.line,
            self.position - self.last_new_line + 1,
            Some("src/main.rs".to_string()),
        );

        match self.c {
            '+' => {
                if self.scan_compound_token(&mut token, '+', TokenType::INCR) {
                    return token;
                }
                token.t = TokenType::PLUS;
            }
            '-' => {
                if self.scan_compound_token(&mut token, '-', TokenType::DECR) {
                    return token;
                }

                if self.scan_compound_token(&mut token, '>', TokenType::ARROW) {
                    return token;
                }

                token.t = TokenType::MINUS;
            }
            '^' => {
                token.t = TokenType::EXPONENT;
            }
            '#' => {
                token.t = TokenType::HASH;
            }
            '&' => {
                if self.scan_compound_token(&mut token, '&', TokenType::AND) {
                    return token;
                }
                token.t = TokenType::AMPERSAND;
            }
            '=' => {
                if self.scan_compound_token(&mut token, '=', TokenType::EQ) {
                    return token;
                }
                token.t = TokenType::ASSIGN;
            }
            '*' => {
                token.t = TokenType::ASTERISK;
            }
            '@' => {
                token.t = TokenType::AT;
            }
            '\\' => {
                token.t = TokenType::BACKSLASH;
            }
            '!' => {
                token.t = TokenType::BANG;
                if self.scan_compound_token(&mut token, '=', TokenType::NEQ) {
                    return token;
                }
            }
            '$' => {
                token.t = TokenType::DOLLAR;
            }
            '.' => {
                if self.scan_compound_token(&mut token, '.', TokenType::RANGE) {
                    return token;
                }
                token.t = TokenType::DOT;
            }
            '"' => {
                token.t = TokenType::STRING;
                token.literal = self.scan_delimiter('"');
            }
            '/' => {
                if self.scan_compound_token(&mut token, '/', TokenType::COMMENT) {
                    self.eat_comment();
                    return self.scan();
                } else if self.scan_compound_token(&mut token, '*', TokenType::COMMENTBLOCK) {
                    self.eat_comment_block();
                    self.advance();
                    return self.scan();
                }
                token.t = TokenType::FORWARDSLASH;
            }
            '%' => {
                token.t = TokenType::PERCENT;
            }
            '|' => {
                if self.scan_compound_token(&mut token, '|', TokenType::OR) {
                    return token;
                }
                token.t = TokenType::PIPE;
            }
            '?' => {
                token.t = TokenType::QUESTION;
            }
            '~' => {
                token.t = TokenType::TILDE;
            }
            '\'' => {
                token.t = TokenType::SQUOTE;
            }
            '(' => {
                token.t = TokenType::LPAREN;
            }
            ')' => {
                token.t = TokenType::RPAREN;
            }
            '{' => {
                token.t = TokenType::LBRACE;
            }
            '}' => {
                token.t = TokenType::RBRACE;
            }
            '[' => {
                token.t = TokenType::LBRACK;
            }
            ']' => {
                token.t = TokenType::RBRACK;
            }
            ',' => {
                token.t = TokenType::COMMA;
            }
            ':' => {
                if self.scan_compound_token(&mut token, ':', TokenType::SCOPE) {
                    return token;
                }
                token.t = TokenType::COLON;
            }
            ';' => {
                token.t = TokenType::SEMICOLON;
            }
            '>' => {
                if self.scan_compound_token(&mut token, '>', TokenType::RSHIFT) {
                    return token;
                } else if self.scan_compound_token(&mut token, '=', TokenType::GTE) {
                    return token;
                }

                token.t = TokenType::GT;
            }
            '<' => {
                if self.scan_compound_token(&mut token, '<', TokenType::LSHIFT) {
                    return token;
                } else if self.scan_compound_token(&mut token, '=', TokenType::LTE) {
                    return token;
                }
                token.t = TokenType::LT;
            }
            // Alphanumeric and Identifier tokens
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.scan_identifier();

                if let Some(keyword) = KeywordTokenType::from_str(&identifier) {
                    token.t = TokenType::KEYWORD(keyword);
                } else {
                    token.t = TokenType::IDENT;
                }

                token.literal = identifier;

                return token;
            }
            // Number tokens
            '0'..='9' => {
                token.t = TokenType::INT;
                token.literal = self.scan_number();

                return token;
            }
            // End-of-file token
            '\0' => {
                token.t = TokenType::EOF;
            }
            // Default case for any other character
            _ => token.t = TokenType::ILLEGAL,
        }

        self.advance();

        token
    }

    pub fn advance(&mut self) {
        if self.next_position >= self.input.len() {
            self.c = '\0';
        } else {
            self.c = self.input[self.next_position];
        }

        self.position = self.next_position;
        self.next_position += 1;
    }

    fn scan_identifier(&mut self) -> String {
        let position = self.position;

        while self.c.is_alphanumeric() || self.c == '_' {
            self.advance();
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn scan_number(&mut self) -> String {
        let position = self.position;

        while self.c.is_numeric() {
            self.advance();
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn scan_delimiter(&mut self, expected_closing: char) -> String {
        let position = self.next_position;

        loop {
            self.advance();

            if self.c == '\0' || self.c == expected_closing {
                break;
            }
        }

        self.input[position..self.position]
            .into_iter()
            .collect::<String>()
    }

    fn scan_compound_token(
        &mut self,
        token: &mut Token,
        next_c: char,
        expected_token: TokenType,
    ) -> bool {
        if self.peek() == next_c {
            let c = self.c;
            self.advance();

            token.t = expected_token;
            token.literal = format!("{}{}", c, self.c);

            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_whitespace(&mut self) {
        loop {
            match self.c {
                ' ' | '\t' => {
                    self.advance();
                }
                '\n' | '\r' => {
                    self.line += 1;
                    self.last_new_line = self.position;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn eat_comment(&mut self) {
        loop {
            match self.c {
                '\n' | '\r' | '\0' => {
                    break;
                }
                _ => self.advance(),
            }
        }
    }

    fn eat_comment_block(&mut self) {
        loop {
            match self.c {
                '\0' => break,
                '*' => {
                    if self.peek() == '/' {
                        self.advance();
                        break;
                    }

                    self.advance()
                }
                _ => self.advance(),
            }
        }
    }

    fn peek(&self) -> char {
        if self.next_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_position]
        }
    }
}

#[cfg(test)]
mod lexer_tests;
