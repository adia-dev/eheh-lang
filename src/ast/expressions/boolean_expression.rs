use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct BooleanExpression {
    pub token: Token,
    pub value: bool,
}

impl BooleanExpression {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }

    pub fn from_token(token: &Token) -> Self {
        let value = token.literal.to_owned();

        Self {
            token: token.clone(),
            value: value == "true",
        }
    }
}

impl Expression for BooleanExpression {
    fn eval(&self) -> String {
        self.get_token_literal()
    }
}

impl Node for BooleanExpression {
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

impl ToString for BooleanExpression {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
