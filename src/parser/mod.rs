use crate::{
    ast::{expressions::identifier::Identifier, statements::declare_statements::DeclareStatement},
    lexer::Lexer,
    program::Program,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
    traits::statement::Statement,
    types::Result,
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.scan();
        let next_token = lexer.scan();

        Self {
            lexer,
            current_token,
            next_token,
        }
    }

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

    fn parse(&mut self) -> Result<Program> {
        let mut new_program = Program::new();

        loop {
            match self.current_token.t {
                TokenType::EOF | TokenType::ILLEGAL => break,
                _ => {
                    match self.parse_statement() {
                        Ok(stmt) => {
                            new_program.statements.push(stmt);
                        }
                        Err(err) => return Err(err),
                    }

                    self.next_token();
                }
            }
        }

        Ok(new_program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>> {
        match self.current_token.t {
            TokenType::KEYWORD(KeywordTokenType::LET)
            | TokenType::KEYWORD(KeywordTokenType::CONST)
            | TokenType::KEYWORD(KeywordTokenType::VAR) => self.parse_declare_statement(),
            _ => Err("".into()),
        }
    }

    fn parse_declare_statement(&mut self) -> Result<Box<dyn Statement>> {
        let current_token = &self.current_token.clone();

        if !self.next_token_is(TokenType::IDENT) {
            return Err(format!(
                "Parsing Error: Expected token {} got {}",
                TokenType::IDENT,
                self.next_token.t
            )
            .into());
        }

        self.next_token();

        let identifier = Identifier::from_token(&self.current_token);
        let type_specifier = self.parse_type_specifier();

        if self.next_token_is(TokenType::ASSIGN) {
            self.next_token();
        }

        loop {
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
            if self.next_token_is(TokenType::IDENT) {
                self.next_token();
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
        ast::statements::declare_statements::DeclareStatement, lexer::Lexer, parser::Parser,
    };

    #[test]
    fn test_declare_statements() {
        const CODE: &'static str = r#"
            let x = 0;
            let y;
            const NUMBER_OF_ROWS: i32 = 100;
            var notifier = null;
            let first_name: string = "Abdoulaye Dia";
        "#;

        let mut lexer = Lexer::new(CODE);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse().unwrap();

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
}
