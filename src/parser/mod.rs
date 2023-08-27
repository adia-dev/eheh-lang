use std::{collections::HashMap, fmt::Debug};

use crate::{
    ast::{
        expressions::{
            boolean_expression::BooleanExpression, call_expression::CallExpression,
            function_literal::FunctionLiteral, identifier::Identifier, if_expression::IfExpression,
            infix_expression::InfixExpression, integer_literal::IntegerLiteral,
            null_expression::NullExpression, prefix_expression::PrefixExpression,
            typed_identifier::TypedIdentifier,
        },
        precedence::Precedence,
        statements::{
            block_statement::BlockStatement, declare_statement::DeclareStatement,
            expression_statement::ExpressionStatement, return_statement::ReturnStatement,
        },
    },
    lexer::Lexer,
    log::{
        error::{ParserError, ParserErrorCode},
        warning::ParserWarning,
    },
    program::Program,
    token::{
        token_type::{KeywordTokenType, TokenType},
        Token,
    },
    types::{
        ASTExpression, ASTExpressionResult, ASTStatement, ASTStatementResult, InfixParseFn,
        ParserResult, PrefixParseFn, Result,
    },
};

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<ParserError>,
    pub warnings: Vec<ParserWarning>,
    prefix_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_fns: HashMap<TokenType, InfixParseFn<'a>>,
    current_delimiter: Option<TokenType>,
    dbg_indent: usize,
    dbg_tracing_enabled: bool,
}

impl<'a> Debug for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.scan();
        let peek_token = lexer.scan();
        let prefix_fns = HashMap::new();
        let infix_fns = HashMap::new();

        let mut parser = Self {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            warnings: Vec::new(),
            prefix_fns,
            infix_fns,
            current_delimiter: None,
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
            .insert(TokenType::KEYWORD(KeywordTokenType::NULL), Self::parse_null);

        self.prefix_fns
            .insert(TokenType::LPAREN, Self::parse_grouped_expression);

        self.prefix_fns
            .insert(TokenType::INT, Self::parse_integer_literal);

        self.prefix_fns.insert(
            TokenType::KEYWORD(KeywordTokenType::IF),
            Self::parse_if_expression,
        );

        self.prefix_fns.insert(
            TokenType::KEYWORD(KeywordTokenType::FUN),
            Self::parse_function_literal,
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
            TokenType::DASTERISK,
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

        self.infix_fns
            .insert(TokenType::LPAREN, Self::parse_call_expression);
    }

    fn advance_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.scan();
        self.maybe_save_current_delimiter();
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.t == t
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.t == t
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.peek_token.t)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.current_token.t)
    }

    fn maybe_save_current_delimiter(&mut self) {
        self.current_delimiter = match self.current_token.t {
            TokenType::COMMENTBLOCK => Some(TokenType::COMMENTBLOCK),
            TokenType::DQUOTE => Some(TokenType::DQUOTE),
            TokenType::LBRACE => Some(TokenType::LBRACE),
            TokenType::LBRACK => Some(TokenType::LBRACK),
            TokenType::LPAREN => Some(TokenType::LPAREN),
            TokenType::PIPE => Some(TokenType::PIPE),
            TokenType::RBRACE => Some(TokenType::RBRACE),
            TokenType::RBRACK => Some(TokenType::RBRACK),
            TokenType::RPAREN => Some(TokenType::RPAREN),
            TokenType::SQUOTE => Some(TokenType::SQUOTE),
            _ => None,
        }
    }

    fn expect_token(&mut self, t: TokenType) -> bool {
        if self.current_token_is(t.clone()) {
            self.advance_token();
            true
        } else {
            false
        }
    }

    fn expect_peek_token_to_be(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.advance_token();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut new_program = Program::new();

        loop {
            match self.current_token.t {
                TokenType::EOF | TokenType::ILLEGAL => break,
                TokenType::SEMICOLON => {
                    self.advance_token();
                    continue;
                }
                TokenType::LBRACE => match self.parse_block_statement() {
                    Ok(stmt) => {
                        new_program.statements.push(Box::new(stmt));
                        self.dbg_untrace("parse_block_statement");
                    }
                    Err(err) => {
                        self.errors.push(err);
                        let current_line = self.current_token.line;
                        while current_line == self.current_token.line
                            && !self.current_token_is(TokenType::EOF)
                        {
                            self.advance_token();
                        }
                    }
                },
                _ => {
                    self.dbg_trace(
                        format!("parse_statement {}", new_program.statements.len() + 1).as_str(),
                    );
                    match self.parse_statement() {
                        Ok(stmt) => {
                            new_program.statements.push(stmt);
                            self.dbg_untrace("parse_statement");
                        }
                        Err(err) => {
                            self.errors.push(err);
                            let current_line = self.current_token.line;
                            while current_line == self.current_token.line {
                                self.advance_token();
                            }
                        }
                    }

                    self.advance_token();
                }
            }
        }

        Ok(new_program)
    }

    fn parse_statement(&mut self) -> ASTStatementResult {
        match self.current_token.t {
            TokenType::KEYWORD(KeywordTokenType::LET)
            | TokenType::KEYWORD(KeywordTokenType::CONST)
            | TokenType::KEYWORD(KeywordTokenType::VAR) => self.parse_declare_statement(),
            TokenType::KEYWORD(KeywordTokenType::RETURN) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_identifier(&mut self) -> ASTExpressionResult {
        self.dbg_trace_inline("parse_identifier");
        Ok(Box::new(Identifier::from_token(&self.current_token)))
    }

    fn parse_boolean(&mut self) -> ASTExpressionResult {
        self.dbg_trace_inline("parse_boolean");
        Ok(Box::new(BooleanExpression::from_token(&self.current_token)))
    }

    fn parse_null(&mut self) -> ASTExpressionResult {
        self.dbg_trace_inline("parse_null");
        Ok(Box::new(NullExpression::new(self.current_token.clone())))
    }

    fn parse_integer_literal(&mut self) -> ASTExpressionResult {
        self.dbg_trace_inline(
            format!("parse_integer_literal: {}", self.current_token.literal).as_str(),
        );
        Ok(Box::new(IntegerLiteral::from_token(&self.current_token)))
    }

    // TODO: REFACTOR THIS DISGUSTING FUNCTION PLEASSEE
    fn parse_if_expression(&mut self) -> ASTExpressionResult {
        self.dbg_trace(format!("parse_if_expression: {}", self.current_token.literal).as_str());

        let current_token = self.current_token.clone(); // if
        let mut surrounded_by_paren = false;

        // optional parentheses around condition
        if self.peek_token_is(TokenType::LPAREN) {
            self.advance_token();
            self.warn(ParserWarning::UnnecessaryParentheses {
                token: self.peek_token.clone(),
                context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
            });
            surrounded_by_paren = true;
        } else if !self.peek_token_is(TokenType::IDENT)
            && !self.peek_token_is(TokenType::KEYWORD(KeywordTokenType::TRUE))
            && !self.peek_token_is(TokenType::KEYWORD(KeywordTokenType::FALSE))
        {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.peek_token.clone(),
                    expected_token_types: vec![
                        TokenType::IDENT,
                        TokenType::LPAREN,
                        TokenType::KEYWORD(KeywordTokenType::TRUE),
                        TokenType::KEYWORD(KeywordTokenType::FALSE),
                    ],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        if surrounded_by_paren && self.peek_token_is(TokenType::RPAREN)
            || self.peek_token_is(TokenType::LBRACE)
        {
            return Err(ParserError {
                code: ParserErrorCode::MissingIfCondition {
                    token: self.current_token.clone(),
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        self.advance_token(); // first token of the condition expression

        let condition_token = self.current_token.clone();
        let condition = self.parse_expression(Precedence::LOWEST)?;

        if !surrounded_by_paren && self.peek_token_is(TokenType::RPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::DelimiterMismatch {
                    token: self.current_token.clone(),
                    expected_delimiter: TokenType::LPAREN,
                    current_delimiter: self.current_delimiter.clone(),
                    context: Some(vec![self
                        .lexer
                        .get_line(self.current_token.line)
                        .unwrap_or(String::new())]),
                },
                source: None,
            });
        } else if surrounded_by_paren && !self.peek_token_is(TokenType::RPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::DelimiterMismatch {
                    token: self.peek_token.clone(),
                    expected_delimiter: TokenType::RPAREN,
                    current_delimiter: self.current_delimiter.clone(),
                    context: Some(vec![self
                        .lexer
                        .get_line(self.peek_token.line)
                        .unwrap_or(String::new())]),
                },
                source: None,
            });
        }

        if surrounded_by_paren {
            self.advance_token();
        }

        self.advance_token(); // first token of the condition expression

        if !self.current_token_is(TokenType::LBRACE) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::LBRACE],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        let consequence_token = self.current_token.clone();
        let consequence = self.parse_block_statement()?;

        let mut alternative_token: Option<Token> = None;
        let mut alternative: Option<BlockStatement> = None;

        if self.peek_token_is(TokenType::KEYWORD(KeywordTokenType::ELSE)) {
            self.advance_token();
            self.advance_token();
            alternative_token = Some(self.current_token.clone());
            alternative = Some(self.parse_block_statement()?);
        }

        if let Some(alt) = &alternative {
            if alt.statements.is_empty() && consequence.statements.is_empty() {
                self.warn(ParserWarning::EmptyIfExpression {
                    token: consequence_token,
                    context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
                });
            } else if consequence.statements.is_empty() {
                self.warn(ParserWarning::EmptyIfConsequenceBranch {
                    token: consequence_token,
                    context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
                    has_alternative: true,
                });
            } else if alt.statements.is_empty() {
                self.warn(ParserWarning::EmptyIfAlternativeBranch {
                    token: alternative_token.unwrap(),
                    context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
                });
            }
        } else if consequence.statements.is_empty() {
            self.warn(ParserWarning::EmptyIfConsequenceBranch {
                token: consequence_token,
                context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
                has_alternative: false,
            });
        }

        self.dbg_untrace("parse_if_expression");

        Ok(Box::new(IfExpression::new(
            current_token.clone(),
            condition,
            consequence,
            alternative,
        )))
    }

    fn parse_function_literal(&mut self) -> ASTExpressionResult {
        self.dbg_trace("parse_if_expression");

        let current_token = self.current_token.clone(); // fn

        let mut name: Option<Identifier> = None;

        if self.peek_token_is(TokenType::IDENT) {
            self.advance_token();
            name = Some(Identifier::from_token(&self.current_token));
        }

        if !self.expect_peek_token_to_be(TokenType::LPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::LPAREN],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        let parameters: Vec<TypedIdentifier> = self.parse_function_parameters()?;

        let mut return_type: Option<Identifier> = None;

        if self.peek_token_is(TokenType::ARROW) {
            self.advance_token();

            if !self.expect_peek_token_to_be(TokenType::IDENT) {
                return Err(ParserError {
                    code: ParserErrorCode::MissingFnReturnType {
                        token: self.current_token.clone(),
                        context: self.lexer.get_line(self.current_token.line),
                    },
                    source: None,
                });
            }

            return_type = Some(Identifier::from_token(&self.current_token));
        }

        if !self.expect_peek_token_to_be(TokenType::LBRACE) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::LBRACE, TokenType::ARROW],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        let body = self.parse_block_statement()?;

        if body.statements.is_empty() {
            self.warn(ParserWarning::EmptyFunction {
                token: self.current_token.clone(),
                context: vec![self.lexer.get_line(self.current_token.line).unwrap()],
            });
        }

        self.dbg_untrace("parse_function_expression");
        Ok(Box::new(FunctionLiteral::new(
            current_token.clone(),
            name,
            None,
            parameters,
            return_type,
            body,
        )))
    }

    fn parse_function_parameters(&mut self) -> ParserResult<Vec<TypedIdentifier>> {
        let mut parameters: Vec<TypedIdentifier> = Vec::new();

        if self.peek_token_is(TokenType::RPAREN) {
            self.advance_token();
            return Ok(parameters);
        }

        self.advance_token();

        let param_ident = Identifier::from_token(&self.current_token);
        let mut param_t: Option<Identifier> = None;
        if self.peek_token_is(TokenType::COLON) {
            self.advance_token();
            self.advance_token();

            param_t = Some(Identifier::from_token(&self.current_token));
        }

        let param = TypedIdentifier::new(param_ident, param_t);
        parameters.push(param);

        loop {
            if !self.peek_token_is(TokenType::COMMA) {
                break;
            }

            self.advance_token();
            self.advance_token();

            let ident = Identifier::from_token(&self.current_token);

            let mut t: Option<Identifier> = None;
            if self.peek_token_is(TokenType::COLON) {
                self.advance_token();
                self.advance_token();

                t = Some(Identifier::from_token(&self.current_token));
            }

            parameters.push(TypedIdentifier::new(ident, t));
        }

        if !self.expect_peek_token_to_be(TokenType::RPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.peek_token.clone(),
                    expected_token_types: vec![TokenType::RPAREN],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        Ok(parameters)
    }

    fn parse_call_expression(&mut self, function: ASTExpression) -> ASTExpressionResult {
        let args = self.parse_call_arguments()?;
        let call_exp = CallExpression::new(self.current_token.clone(), function, args);
        Ok(Box::new(call_exp))
    }

    fn parse_call_arguments(&mut self) -> ParserResult<Vec<ASTExpression>> {
        let mut args: Vec<ASTExpression> = Vec::new();

        if self.peek_token_is(TokenType::RPAREN) {
            self.advance_token();
            return Ok(args);
        }

        self.advance_token();
        args.push(self.parse_expression(Precedence::LOWEST)?);

        loop {
            if !self.peek_token_is(TokenType::COMMA) {
                break;
            }

            self.advance_token();
            self.advance_token();

            args.push(self.parse_expression(Precedence::LOWEST)?);
        }

        if !self.expect_peek_token_to_be(TokenType::RPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::RPAREN],
                    context: self.lexer.get_line(self.peek_token.line),
                },
                source: None,
            });
        }

        Ok(args)
    }

    // double cloning eww :/
    fn parse_prefix_expression(&mut self) -> ASTExpressionResult {
        self.dbg_trace(format!("parse_prefix_expression: {}", self.current_token.t).as_str());

        let current_token = self.current_token.clone();
        self.advance_token();

        match self.parse_expression(Precedence::PREFIX) {
            Ok(rhs) => {
                self.dbg_untrace("parse_prefix_expression");

                Ok(Box::new(PrefixExpression::new(
                    current_token.clone(),
                    current_token.literal,
                    rhs,
                )))
            }
            Err(err) => {
                self.errors.push(err.clone());
                self.advance_token();
                return Err(err);
            }
        }
    }

    fn parse_infix_expression(&mut self, lhs: ASTExpression) -> ASTExpressionResult {
        self.dbg_trace(format!("parse_infix_expression: {}", self.current_token.t).as_str());

        let precedence = self.current_precedence();
        let current_token = self.current_token.clone();
        self.advance_token();
        let rhs = self.parse_expression(precedence)?;

        self.dbg_untrace("parse_prefix_expression");

        Ok(Box::new(InfixExpression::new(
            current_token.clone(),
            lhs,
            current_token.literal,
            rhs,
        )))
    }

    fn parse_block_statement(&mut self) -> ParserResult<BlockStatement> {
        self.dbg_trace("parse_block_statement");
        let current_token = &self.current_token.clone();
        let mut statements: Vec<ASTStatement> = Vec::new();

        self.advance_token();

        loop {
            if self.current_token_is(TokenType::RBRACE) {
                break;
            } else if self.current_token_is(TokenType::EOF) {
                return Err(ParserError {
                    code: ParserErrorCode::UnexpectedToken {
                        token: self.current_token.clone(),
                        expected_token_types: vec![TokenType::RBRACE],
                        context: self.lexer.get_line(self.current_token.line),
                    },
                    source: None,
                });
            }

            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                _ => (),
            };

            self.advance_token();
        }

        let stmt = BlockStatement::new(current_token.clone(), statements);

        self.dbg_untrace("parse_block_expression");
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> ASTStatementResult {
        self.dbg_trace("parse_return_statement");
        let current_token = &self.current_token.clone();

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.dbg_untrace("parse_return_statement");
            self.advance_token();
            return Ok(Box::new(ReturnStatement::new(current_token.clone(), None)));
        }

        self.advance_token();

        match self.parse_expression(Precedence::LOWEST) {
            Ok(ret_val) => {
                if !self.expect_peek_token_to_be(TokenType::SEMICOLON) {
                    return Err(ParserError {
                        code: ParserErrorCode::UnexpectedToken {
                            token: self.current_token.clone(),
                            expected_token_types: vec![TokenType::IDENT],
                            context: self.lexer.get_line(self.peek_token.line),
                        },
                        source: None,
                    });
                }
                self.dbg_untrace("parse_return_statement");
                Ok(Box::new(ReturnStatement::new(
                    current_token.clone(),
                    Some(ret_val),
                )))
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn parse_expression_statement(&mut self) -> ASTStatementResult {
        self.dbg_trace("parse_expression_statement");
        match self.parse_expression(Precedence::LOWEST) {
            Ok(expression) => {
                let stmt = ExpressionStatement::new(self.current_token.clone(), expression);

                if self.peek_token_is(TokenType::SEMICOLON) {
                    self.advance_token();
                }

                self.dbg_untrace("parse_expression_statement");
                Ok(Box::new(stmt))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ASTExpressionResult {
        self.dbg_trace("parse_expression");
        if let Some(prefix_fn) = self.prefix_fns.get(&self.current_token.t) {
            let mut left_exp = prefix_fn(self)?;

            loop {
                if self.peek_token_is(TokenType::SEMICOLON) {
                    break;
                }

                if self.current_token_is(TokenType::EOF) {
                    return Err(ParserError {
                        code: ParserErrorCode::UnexpectedToken {
                            token: self.current_token.clone(),
                            expected_token_types: vec![TokenType::EOF],
                            context: self.lexer.get_line(self.current_token.line),
                        },
                        source: None,
                    });
                }

                if precedence >= self.peek_precedence() {
                    break;
                }

                if let Some(_infix_fn) = self.infix_fns.get(&self.peek_token.t) {
                    self.advance_token();
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
            Err(ParserError {
                code: ParserErrorCode::UnknownPrefixToken {
                    token: self.current_token.clone(),
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            })
        }
    }

    fn parse_grouped_expression(&mut self) -> ASTExpressionResult {
        self.dbg_trace("parse_grouped_expression");
        self.advance_token();

        let exp = self.parse_expression(Precedence::LOWEST);

        if !self.peek_token_is(TokenType::RPAREN) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::RPAREN],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        self.advance_token();
        self.dbg_untrace("parse_grouped_expression");
        exp
    }

    fn parse_declare_statement(&mut self) -> ASTStatementResult {
        let current_token = &self.current_token.clone();
        self.dbg_trace(
            format!(
                "parse_{}_statement",
                self.current_token.literal.clone().to_lowercase()
            )
            .as_str(),
        );

        if !self.expect_peek_token_to_be(TokenType::IDENT) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::IDENT],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        let identifier = Identifier::from_token(&self.current_token);
        let type_specifier = self.parse_type_specifier();

        if self.peek_token_is(TokenType::ASSIGN) {
            self.advance_token();
        } else if self.peek_token_is(TokenType::SEMICOLON) {
            self.advance_token();

            self.dbg_untrace(
                format!(
                    "parse_{}_statement",
                    current_token.literal.clone().to_lowercase()
                )
                .as_str(),
            );

            return Ok(Box::new(DeclareStatement::new(
                current_token.clone(),
                identifier,
                type_specifier,
                None,
            )));
        } else if !self.peek_token_is(TokenType::ASSIGN) {
            return Err(ParserError {
                code: ParserErrorCode::UnexpectedToken {
                    token: self.current_token.clone(),
                    expected_token_types: vec![TokenType::SEMICOLON],
                    context: self.lexer.get_line(self.current_token.line),
                },
                source: None,
            });
        }

        self.advance_token();

        match self.parse_expression(Precedence::LOWEST) {
            Ok(val) => {
                if !self.expect_peek_token_to_be(TokenType::SEMICOLON) {
                    return Err(ParserError {
                        code: ParserErrorCode::UnexpectedToken {
                            token: self.current_token.clone(),
                            expected_token_types: vec![TokenType::SEMICOLON],
                            context: self.lexer.get_line(self.current_token.line),
                        },
                        source: None,
                    });
                }
                self.dbg_untrace(
                    format!(
                        "parse_{}_statement",
                        current_token.literal.clone().to_lowercase()
                    )
                    .as_str(),
                );
                Ok(Box::new(DeclareStatement::new(
                    current_token.clone(),
                    identifier,
                    type_specifier,
                    Some(val),
                )))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_type_specifier(&mut self) -> Option<String> {
        if self.peek_token_is(TokenType::COLON) {
            self.advance_token();
            if self.expect_peek_token_to_be(TokenType::IDENT) {
                Some(self.current_token.clone().literal)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn warn(&mut self, warning: ParserWarning) {
        self.warnings.push(warning);
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
mod test;
