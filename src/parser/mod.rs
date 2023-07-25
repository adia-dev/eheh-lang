use std::collections::HashMap;

use crate::{
    ast::{
        expressions::{identifier::Identifier, integer_literal::IntegerLiteral},
        precedence::Precedence,
        statements::{
            declare_statements::DeclareStatement, expression_statements::ExpressionStatement,
            return_statements::ReturnStatement,
        },
    },
    lexer::Lexer,
    program::Program,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
    types::{ExpressionResponse, InfixParseFn, PrefixParseFn, Result, StatementResponse},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    next_token: Token,
    errors: Vec<String>,
    prefix_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_fns: HashMap<TokenType, InfixParseFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.scan();
        let next_token = lexer.scan();
        let prefix_fns = HashMap::new();
        let infix_fns = HashMap::new();

        let mut parser = Self {
            lexer,
            current_token,
            next_token,
            errors: Vec::new(),
            prefix_fns,
            infix_fns,
        };

        parser.register_prefix_fns();
        parser.register_infix_fns();

        parser
    }

    fn register_prefix_fns(&mut self) {
        self.prefix_fns
            .insert(TokenType::IDENT, Self::parse_identifier);
        self.prefix_fns
            .insert(TokenType::INT, Self::parse_integer_literal);
    }

    fn register_infix_fns(&mut self) {}

    fn next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.scan();
    }

    fn next_token_is(&self, t: TokenType) -> bool {
        self.next_token.t == t
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.t == t
    }

    fn unexpected_error(&mut self, expected: &str, got: Token) {
        self.errors.push(format!(
            "Expected token to be {}, got {} instead at {}:{}",
            expected, got.t, got.line, got.position
        ))
    }

    fn expect_token(&mut self, t: TokenType) -> bool {
        if self.current_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.errors.push(format!(
                "Expected token to be {}, got {} instead at {}:{}",
                t, self.current_token.t, self.current_token.line, self.current_token.position
            ));
            false
        }
    }

    fn expect_next_token(&mut self, t: TokenType) -> bool {
        if self.next_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.errors.push(format!(
                "Expected token to be {}, got {} instead at {}:{}",
                t, self.next_token.t, self.next_token.line, self.next_token.position
            ));
            false
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut new_program = Program::new();

        loop {
            match self.current_token.t {
                TokenType::EOF | TokenType::ILLEGAL => break,
                TokenType::SEMICOLON => {
                    self.next_token();
                    continue;
                }
                _ => {
                    match self.parse_statement() {
                        Ok(stmt) => {
                            new_program.statements.push(stmt);
                        }
                        Err(_err) => (),
                    }

                    self.next_token();
                }
            }
        }

        Ok(new_program)
    }

    fn parse_statement(&mut self) -> StatementResponse {
        match self.current_token.t {
            TokenType::KEYWORD(KeywordTokenType::LET)
            | TokenType::KEYWORD(KeywordTokenType::CONST)
            | TokenType::KEYWORD(KeywordTokenType::VAR) => self.parse_declare_statement(),
            TokenType::KEYWORD(KeywordTokenType::RETURN) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_identifier(&mut self) -> ExpressionResponse {
        Ok(Box::new(Identifier::from_token(&self.current_token)))
    }

    fn parse_integer_literal(&mut self) -> ExpressionResponse {
        Ok(Box::new(IntegerLiteral::from_token(&self.current_token)))
    }

    fn parse_return_statement(&mut self) -> StatementResponse {
        let current_token = &self.current_token.clone();

        loop {
            if self.current_token_is(TokenType::SEMICOLON) {
                break;
            } else if self.current_token_is(TokenType::EOF) {
                self.unexpected_error("EXPR | SEMICOLON", self.current_token.clone());
                break;
            }

            self.next_token();
        }

        let stmt = ReturnStatement::new(current_token.clone(), None);

        Ok(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> StatementResponse {
        match self.parse_expression(Precedence::LOWEST) {
            Ok(expression) => {
                let stmt = ExpressionStatement::new(self.current_token.clone(), expression);

                if self.next_token_is(TokenType::SEMICOLON) {
                    self.next_token();
                }

                Ok(Box::new(stmt))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> ExpressionResponse {
        if let Some(prefix_fn) = self.prefix_fns.get(&self.current_token.t) {
            let left_exp = prefix_fn(self);

            left_exp
        } else {
            self.errors.push(format!(
                "Parsing Error: Could not find a prefix parsing function for {}",
                self.current_token.t
            ));

            Err(format!(
                "Parsing Error: Could not find a prefix parsing function for {}",
                self.current_token.t
            )
            .into())
        }
    }

    fn parse_declare_statement(&mut self) -> StatementResponse {
        let current_token = &self.current_token.clone();

        if !self.expect_next_token(TokenType::IDENT) {
            return Err(format!(
                "Parsing Error: Expected token {} got {} at {}:{}",
                TokenType::IDENT,
                self.next_token.t,
                self.next_token.line,
                self.next_token.position,
            )
            .into());
        }

        let identifier = Identifier::from_token(&self.current_token);
        let type_specifier = self.parse_type_specifier();

        if self.next_token_is(TokenType::ASSIGN) {
            self.next_token();
        } else if !self.next_token_is(TokenType::SEMICOLON) {
            self.expect_next_token(TokenType::ASSIGN);
        }

        loop {
            if self.current_token_is(TokenType::EOF) {
                self.errors.push(format!(
                    "Expected next token to be {}, got EOF instead at {}:{}",
                    TokenType::SEMICOLON,
                    current_token.line,
                    current_token.position
                ));
                break;
            }
            if self.current_token_is(TokenType::SEMICOLON) {
                break;
            }
            self.next_token();
        }

        let stmt = DeclareStatement::new(current_token.clone(), identifier, type_specifier, None);

        Ok(Box::new(stmt))
    }

    fn parse_type_specifier(&mut self) -> Option<String> {
        if self.next_token_is(TokenType::COLON) {
            self.next_token();
            if self.expect_next_token(TokenType::IDENT) {
                Some(self.current_token.clone().literal)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            expressions::{identifier::Identifier, integer_literal::IntegerLiteral},
            statements::{
                declare_statements::DeclareStatement, expression_statements::ExpressionStatement,
            },
        },
        lexer::Lexer,
        parser::Parser,
        token::token_type::TokenType,
        traits::node::Node,
    };

    #[test]
    fn test_declare_statements() {
        const CODE: &'static str = r#"
            let x = 0;
            let y;
            const NUMBER_OF_ROWS: i32 = 100;;;
            var notifier = null;
            let first_name: string = "Abdoulaye Dia";
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 5);

        let expected_identifiers: Vec<&str> =
            vec!["x", "y", "NUMBER_OF_ROWS", "notifier", "first_name"];

        expected_identifiers
            .iter()
            .enumerate()
            .for_each(|(i, identifier)| {
                if let Some(stmt) = program.statements.get(i) {
                    match stmt.as_any().downcast_ref::<DeclareStatement>() {
                        Some(declare_stmt) => {
                            assert_eq!(identifier.to_owned(), declare_stmt.name.value);
                        }
                        _ => panic!("Parsing Error: Expected to receive a Declare Statement."),
                    }
                }
            });
    }

    #[test]
    fn test_return_statement() {
        const CODE: &'static str = r#"
            return x;
            return y + b;
            return;
            return ("Abdoulaye Dia");
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 4);
    }

    #[test]
    fn test_expression_statement_with_identifier() {
        const CODE: &'static str = r#"
            name;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let exp_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();
        let exp_ident = exp_stmt
            .expression
            .as_any()
            .downcast_ref::<Identifier>()
            .unwrap();

        assert_eq!(exp_ident.token.t, TokenType::IDENT);
        assert_eq!(exp_ident.get_token_literal(), "name");

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_expression_statement_with_integer_literal() {
        const CODE: &'static str = r#"
            5;
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

        let exp_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .unwrap();

        let exp_int = exp_stmt
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .unwrap();

        assert_eq!(exp_int.token.t, TokenType::INT);
        assert_eq!(exp_int.get_token_literal(), "5");
        assert_eq!(exp_int.value, 5);

        assert!(parser.errors.is_empty());
        assert_eq!(program.statements.len(), 1);
    }
}
