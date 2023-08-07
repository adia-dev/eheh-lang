use std::collections::HashMap;

use crate::{
    ast::{
        expressions::{
            boolean::Boolean, identifier::Identifier, if_expression::IfExpression,
            infix_expression::InfixExpression, integer_literal::IntegerLiteral,
            prefix_expression::PrefixExpression,
        },
        precedence::Precedence,
        statements::{
            block_statement::BlockStatement, declare_statements::DeclareStatement,
            expression_statements::ExpressionStatement, return_statements::ReturnStatement,
        },
    },
    lexer::Lexer,
    program::Program,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
    traits::{expression::Expression, statement::Statement},
    types::{ExpressionResult, InfixParseFn, PrefixParseFn, Result, StatementResult},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    next_token: Token,
    errors: Vec<String>,
    warnings: Vec<String>,
    prefix_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_fns: HashMap<TokenType, InfixParseFn<'a>>,
    dbg_indent: usize,
    dbg_tracing_enabled: bool,
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
            warnings: Vec::new(),
            prefix_fns,
            infix_fns,
            dbg_indent: 0,
            dbg_tracing_enabled: false,
        };

        parser.register_prefix_fns();
        parser.register_infix_fns();

        parser
    }

    fn register_prefix_fns(&mut self) {
        self.prefix_fns
            .insert(TokenType::IDENT, Self::parse_identifier);

        self.prefix_fns.insert(
            TokenType::KEYWORD(KeywordTokenType::TRUE),
            Self::parse_boolean,
        );
        self.prefix_fns.insert(
            TokenType::KEYWORD(KeywordTokenType::FALSE),
            Self::parse_boolean,
        );

        self.prefix_fns
            .insert(TokenType::LPAREN, Self::parse_grouped_expression);

        self.prefix_fns
            .insert(TokenType::INT, Self::parse_integer_literal);

        self.prefix_fns.insert(
            TokenType::KEYWORD(KeywordTokenType::IF),
            Self::parse_if_expression,
        );

        let prefix_tokens: Vec<TokenType> = vec![
            TokenType::INCR,
            TokenType::DECR,
            TokenType::BANG,
            TokenType::MINUS,
            TokenType::RANGE,
            TokenType::IRANGE,
        ];

        for t in &prefix_tokens {
            self.prefix_fns
                .insert(t.clone(), Self::parse_prefix_expression);
        }
    }

    fn register_infix_fns(&mut self) {
        let infix_tokens: Vec<TokenType> = vec![
            TokenType::PLUS,
            TokenType::MINUS,
            TokenType::ASTERISK,
            TokenType::FORWARDSLASH,
            TokenType::IRANGE,
            TokenType::RANGE,
            TokenType::EXPONENT,
            TokenType::AND,
            TokenType::OR,
            TokenType::LT,
            TokenType::LTE,
            TokenType::GT,
            TokenType::GTE,
            TokenType::EQ,
            TokenType::NEQ,
            TokenType::LSHIFT,
            TokenType::RSHIFT,
            TokenType::PERCENT,
        ];

        for t in &infix_tokens {
            self.infix_fns
                .insert(t.clone(), Self::parse_infix_expression);
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

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.next_token.t)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.current_token.t)
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
                    self.dbg_trace(
                        format!("parse_statement {}", new_program.statements.len() + 1).as_str(),
                    );
                    match self.parse_statement() {
                        Ok(stmt) => {
                            new_program.statements.push(stmt);
                            self.dbg_untrace("parse_statement");
                        }
                        Err(_err) => (),
                    }

                    self.next_token();
                }
            }
        }

        Ok(new_program)
    }

    fn parse_statement(&mut self) -> StatementResult {
        match self.current_token.t {
            TokenType::KEYWORD(KeywordTokenType::LET)
            | TokenType::KEYWORD(KeywordTokenType::CONST)
            | TokenType::KEYWORD(KeywordTokenType::VAR) => self.parse_declare_statement(),
            TokenType::KEYWORD(KeywordTokenType::RETURN) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_identifier(&mut self) -> ExpressionResult {
        self.dbg_trace_inline("parse_identifier");
        Ok(Box::new(Identifier::from_token(&self.current_token)))
    }

    fn parse_boolean(&mut self) -> ExpressionResult {
        self.dbg_trace_inline("parse_boolean");
        Ok(Box::new(Boolean::from_token(&self.current_token)))
    }

    fn parse_integer_literal(&mut self) -> ExpressionResult {
        self.dbg_trace_inline(
            format!("parse_integer_literal: {}", self.current_token.literal).as_str(),
        );
        Ok(Box::new(IntegerLiteral::from_token(&self.current_token)))
    }

    fn parse_if_expression(&mut self) -> ExpressionResult {
        self.dbg_trace(format!("parse_if_expression: {}", self.current_token.literal).as_str());

        let current_token = self.current_token.clone(); // if
        let mut surrounded_by_paren = false;

        // optional parentheses around condition
        if self.next_token_is(TokenType::LPAREN) {
            self.next_token();
            self.warn("unnecessary parentheses around `if` condition");
            surrounded_by_paren = true;
        }

        if surrounded_by_paren && self.next_token_is(TokenType::RPAREN)
            || self.next_token_is(TokenType::LBRACE)
        {
            self.errors.push("Parsing Error: If expressions need to contain a condition, maybe you forgot it.\nreminder: if (cond) { ... } <optional> else { ... }".to_string());

            return Err("Parsing Error: If expressions need to contain a condition, maybe you forgot it.\nrto_stringeminder: if (cond) { ... } <optional> else { ... }".into());
        }

        self.next_token(); // first token of the condition expression

        let condition = self.parse_expression(Precedence::LOWEST)?;

        self.next_token();

        if surrounded_by_paren && !self.expect_token(TokenType::RPAREN) {
            self.errors.push(format!(
                "Parsing Error: Could not parse an if expression, expected token RPAREN, got {}",
                self.current_token.t
            ));

            return Err(format!(
                "Parsing Error: Could not parse an if expression, expected token RPAREN, got {}",
                self.current_token.t
            )
            .into());
        }

        if !self.current_token_is(TokenType::LBRACE) {
            self.errors.push(format!(
                "Parsing Error: Could not parse an if expression, expected token LBRACE, got {}",
                self.current_token.t
            ));

            return Err(format!(
                "Parsing Error: Could not parse an if expression, expected token LBRACE, got {}",
                self.current_token.t
            )
            .into());
        }

        let consequence = self.parse_block_statement()?;
        let mut alternative: Option<BlockStatement> = None;

        if self.next_token_is(TokenType::KEYWORD(KeywordTokenType::ELSE)) {
            self.next_token();
            self.next_token();
            alternative = Some(self.parse_block_statement()?);
        }

        if let Some(alt) = &alternative {
            if alt.statements.is_empty() && consequence.statements.is_empty() {
                self.warn("the consequence and the alternative(s) of the if expression are empty, you might want to if expression remove it.");
            } else if consequence.statements.is_empty() {
                self.warn("the consequence of the if statement is empty, you might want to invert the condition and remove the alternative statement like so:\n if !(cond) {{ ... }}");
            } else if alt.statements.is_empty() {
                self.warn("the alternative of the if statement is empty, you might want to remove the alternative statement like so:\n if (cond) {{ ... }}");
            }
        } else if consequence.statements.is_empty() {
            self.warn(
                "the consequence of the if expression is empty, you might want to remove it.",
            );
        }

        self.dbg_untrace("parse_if_expression");

        Ok(Box::new(IfExpression::new(
            current_token.clone(),
            condition,
            consequence,
            alternative,
        )))
    }

    // double cloning eww :/
    fn parse_prefix_expression(&mut self) -> ExpressionResult {
        self.dbg_trace(format!("parse_prefix_expression: {}", self.current_token.t).as_str());

        let current_token = self.current_token.clone();
        self.next_token();
        let rhs = self.parse_expression(Precedence::PREFIX)?;

        self.dbg_untrace("parse_prefix_expression");

        Ok(Box::new(PrefixExpression::new(
            current_token.clone(),
            current_token.literal,
            rhs,
        )))
    }

    fn parse_infix_expression(&mut self, lhs: Box<dyn Expression>) -> ExpressionResult {
        self.dbg_trace(format!("parse_infix_expression: {}", self.current_token.t).as_str());

        let precedence = self.current_precedence();
        let current_token = self.current_token.clone();
        self.next_token();
        let rhs = self.parse_expression(precedence)?;

        self.dbg_untrace("parse_prefix_expression");

        Ok(Box::new(InfixExpression::new(
            current_token.clone(),
            lhs,
            current_token.literal,
            rhs,
        )))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement> {
        self.dbg_trace("parse_block_statement");
        let current_token = &self.current_token.clone();
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();

        self.next_token();

        loop {
            if self.current_token_is(TokenType::RBRACE) {
                break;
            } else if self.current_token_is(TokenType::EOF) {
                self.unexpected_error("RBRACE", self.current_token.clone());
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                _ => (),
            };

            self.next_token();
        }

        let stmt = BlockStatement::new(current_token.clone(), statements);

        self.dbg_untrace("parse_block_expression");
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> StatementResult {
        self.dbg_trace("parse_return_statement");
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

        self.dbg_untrace("parse_return_statement");
        Ok(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> StatementResult {
        self.dbg_trace("parse_expression_statement");
        match self.parse_expression(Precedence::LOWEST) {
            Ok(expression) => {
                let stmt = ExpressionStatement::new(self.current_token.clone(), expression);

                if self.next_token_is(TokenType::SEMICOLON) {
                    self.next_token();
                }

                self.dbg_untrace("parse_expression_statement");
                Ok(Box::new(stmt))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ExpressionResult {
        self.dbg_trace("parse_expression");
        if let Some(prefix_fn) = self.prefix_fns.get(&self.current_token.t) {
            let mut left_exp = prefix_fn(self)?;

            loop {
                if self.next_token_is(TokenType::SEMICOLON) || self.next_token_is(TokenType::EOF) {
                    break;
                }

                if precedence >= self.peek_precedence() {
                    break;
                }

                if let Some(_infix_fn) = self.infix_fns.get(&self.next_token.t) {
                    self.next_token();
                } else {
                    break;
                }

                if let Some(infix_fn) = self.infix_fns.get(&self.current_token.t) {
                    left_exp = infix_fn(self, left_exp)?;
                } else {
                    break;
                }
            }

            self.dbg_untrace("parse_expression");
            Ok(left_exp)
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

    fn parse_grouped_expression(&mut self) -> ExpressionResult {
        self.dbg_trace("parse_grouped_expression");
        self.next_token();

        let exp = self.parse_expression(Precedence::LOWEST);

        if !self.next_token_is(TokenType::RPAREN) {
            self.errors.push(format!(
                "Parsing Error: Could not parse a grouped expression, expected token RPAREN, got {}",
                self.current_token.t
            ));

            return Err(format!(
                "Parsing Error: Could not parse a grouped expression, expected token RPAREN, got {}",
                self.current_token.t
            )
            .into());
        }

        self.next_token();
        self.dbg_untrace("parse_grouped_expression");
        exp
    }

    fn parse_declare_statement(&mut self) -> StatementResult {
        let current_token = &self.current_token.clone();
        self.dbg_trace(
            format!(
                "parse_{}_statement",
                self.current_token.literal.clone().to_lowercase()
            )
            .as_str(),
        );

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

        self.dbg_untrace(
            format!(
                "parse_{}_statement",
                current_token.literal.clone().to_lowercase()
            )
            .as_str(),
        );
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

    fn warn(&mut self, text: &str) {
        self.warnings.push(format!(
            "Warning({}): {}",
            self.current_token.get_location(),
            text
        ));
    }

    fn dbg_trace(&mut self, context: &str) {
        if !self.dbg_tracing_enabled {
            return;
        }
        let padding = "    ".repeat(self.dbg_indent);
        println!("{}BEGIN {}", padding, context);
        self.dbg_indent += 1;
    }

    fn dbg_trace_inline(&mut self, context: &str) {
        if !self.dbg_tracing_enabled {
            return;
        }
        let padding = "    ".repeat(self.dbg_indent);
        println!("{}ACTION {}", padding, context);
    }

    fn dbg_untrace(&mut self, context: &str) {
        if !self.dbg_tracing_enabled {
            return;
        }
        if self.dbg_indent > 0 {
            self.dbg_indent -= 1;
        }
        let padding = "    ".repeat(self.dbg_indent);
        println!("{}END {}", padding, context);
    }
}

#[cfg(test)]
mod parser_tests;
