use crate::{
    ast::statements::block_statement::BlockStatement,
    token::Token,
    traits::{expression::Expression, node::Node},
};

use super::{identifier::Identifier, typed_identifier::TypedIdentifier};

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub content: String,
}

impl StringLiteral {
    pub fn new(token: Token, content: &str) -> Self {
        Self {
            token,
            content: content.to_owned(),
        }
    }
}

impl Expression for StringLiteral {
    fn eval(&self) -> String {
        "".to_string()
    }
    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for StringLiteral {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_node(&self) -> &dyn Node {
        self
    }
}

impl ToString for StringLiteral {
    fn to_string(&self) -> String {
        format!("{}", self.content)
    }
}
