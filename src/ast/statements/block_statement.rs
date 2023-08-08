use std::any::Any;

use crate::{
    token::Token,
    traits::{node::Node, statement::Statement},
    types::ASTStatement,
};

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<ASTStatement>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<ASTStatement>) -> Self {
        Self { token, statements }
    }
}

impl Node for BlockStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for BlockStatement {
    fn process(&self) {}
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        let mut statements_as_str = String::new();

        for s in &self.statements {
            statements_as_str.push_str("    ");
            statements_as_str.push_str(s.to_string().as_str())
        }

        format!("{{\n{}\n}}", statements_as_str)
    }
}
