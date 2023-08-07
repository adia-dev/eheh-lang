use std::any::Any;

use crate::{
    ast::expressions::identifier::Identifier,
    token::Token,
    traits::{expression::Expression, node::Node, statement::Statement},
};

#[derive(Debug)]
pub struct DeclareStatement {
    pub token: Token,
    pub name: Identifier,
    pub type_specifier: Option<String>,
    pub value: Option<Box<dyn Expression>>,
}

impl DeclareStatement {
    pub fn new(
        token: Token,
        name: Identifier,
        type_specifier: Option<String>,
        value: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            name,
            type_specifier,
            value,
        }
    }
}

impl Node for DeclareStatement {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for DeclareStatement {
    fn process(&self) {}
}

impl ToString for DeclareStatement {
    fn to_string(&self) -> String {
        if let Some(type_specifier) = &self.type_specifier {
            if let Some(value) = &self.value {
                format!(
                    "{} {}: {} = {};",
                    self.token.literal,
                    self.name.get_token_literal(),
                    type_specifier,
                    value.to_string()
                )
            } else {
                format!(
                    "{} {}: {};",
                    self.token.literal,
                    self.name.get_token_literal(),
                    type_specifier
                )
            }
        } else {
            if let Some(value) = &self.value {
                format!(
                    "{} {} = {};",
                    self.token.literal,
                    self.name.get_token_literal(),
                    value.to_string()
                )
            } else {
                format!("{} {};", self.token.literal, self.name.get_token_literal())
            }
        }
    }
}
