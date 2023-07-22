use crate::token::{
    token_type::{KeywordTokenType, TokenType},
    Token,
};

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub next_position: usize,
    pub c: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut new_lexer = Self {
            input: input.chars().into_iter().collect(),
            position: 0,
            next_position: 0,
            c: '\0',
        };

        new_lexer.advance();

        new_lexer
    }

    pub fn scan(&mut self) -> Token {
        self.eat_whitespace();

        let mut token = Token::new(TokenType::ILLEGAL, self.c.to_string());

        match self.c {
            '=' => {
                token.t = TokenType::ASSIGN;
            }
            '+' => {
                token.t = TokenType::PLUS;
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
                token.t = TokenType::COLON;
            }
            ';' => {
                token.t = TokenType::SEMICOLON;
            }
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
            '0'..='9' => {
                token.t = TokenType::INT;
                token.literal = self.scan_number();
                return token;
            }
            '\0' => {
                token.t = TokenType::EOF;
            }
            _ => token.literal = self.c.to_string(),
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

    fn eat_whitespace(&mut self) {
        loop {
            match self.c {
                ' ' | '\t' => {
                    self.advance();
                }
                '\n' | '\r' => {
                    // TODO: increase the current line right there
                    self.advance();
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::Lexer,
        token::token_type::{
            KeywordTokenType::{self, FUN},
            TokenType,
        },
    };

    #[test]
    fn ping() {
        println!("pong !");
        assert!(true)
    }

    #[test]
    fn test_next_token() {
        const CODE: &'static str = "=+(){}[],:;";

        let mut expected_tokens: Vec<(TokenType, String)> = Vec::new();
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::LBRACK, "[".to_owned()));
        expected_tokens.push((TokenType::RBRACK, "]".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::COLON, ":".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::EOF, "\0".to_owned()));

        let mut lexer = Lexer::new(CODE.as_ref());

        for (t, literal) in expected_tokens {
            let token = lexer.scan();

            assert_eq!(token.t, t, "Expected token {} got {}", t, token.t);
            assert_eq!(
                token.t, t,
                "Expected literal {} got {}",
                literal, token.literal
            );
        }
    }

    #[test]
    fn test_next_token_with_code() {
        const CODE: &'static str = r#"
        let five = 5;
        const ten = 10;

        let add = fn(x, y) {
            x + y
        };

        let result = add(five, ten);
    "#;

        let mut expected_tokens: Vec<(TokenType, String)> = Vec::new();
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "5".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((
            TokenType::KEYWORD(KeywordTokenType::CONST),
            "const".to_owned(),
        ));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::INT, "10".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::FUN), "fn".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::LBRACE, "{".to_owned()));
        expected_tokens.push((TokenType::IDENT, "x".to_owned()));
        expected_tokens.push((TokenType::PLUS, "+".to_owned()));
        expected_tokens.push((TokenType::IDENT, "y".to_owned()));
        expected_tokens.push((TokenType::RBRACE, "}".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::KEYWORD(KeywordTokenType::LET), "let".to_owned()));
        expected_tokens.push((TokenType::IDENT, "result".to_owned()));
        expected_tokens.push((TokenType::ASSIGN, "=".to_owned()));
        expected_tokens.push((TokenType::IDENT, "add".to_owned()));
        expected_tokens.push((TokenType::LPAREN, "(".to_owned()));
        expected_tokens.push((TokenType::IDENT, "five".to_owned()));
        expected_tokens.push((TokenType::COMMA, ",".to_owned()));
        expected_tokens.push((TokenType::IDENT, "ten".to_owned()));
        expected_tokens.push((TokenType::RPAREN, ")".to_owned()));
        expected_tokens.push((TokenType::SEMICOLON, ";".to_owned()));
        expected_tokens.push((TokenType::EOF, "\0".to_owned()));

        let mut lexer = Lexer::new(CODE.as_ref());

        for (t, literal) in expected_tokens {
            let token = lexer.scan();

            assert_eq!(
                token.t, t,
                "\n\nExpected token {:?} got {:?}.\n\n",
                t, token.t
            );
            assert_eq!(
                token.literal, literal,
                "Expected literal {} got {}",
                literal, token.literal
            );
        }
    }
}
