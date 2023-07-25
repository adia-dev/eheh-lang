use std::any::Any;

use crate::{
    token::Token,
    traits::{expression::Expression, node::Node, statement::Statement},
};

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> Self {
        Self { token, value }
    }
}

impl Node for ReturnStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn process(&self) {}
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        format!("return {:?};", self.value)
    }
}
