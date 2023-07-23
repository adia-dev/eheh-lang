use crate::{
    ast::expressions::identifier_expression::IdentifierExpression,
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct DeclareStatement {
    pub token: Token,
    pub name: IdentifierExpression,
    pub type_specifier: Option<String>,
    pub value: Option<Box<dyn Expression>>,
}

impl DeclareStatement {
    pub fn new(
        token: Token,
        name: IdentifierExpression,
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
}

impl ToString for DeclareStatement {
    fn to_string(&self) -> String {
        self.token.literal.to_string()
    }
}
