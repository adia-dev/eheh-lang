use crate::{
    token::Token,
    traits::{expression::Expression, node::Node},
};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }

    pub fn from_token(token: &Token) -> Self {
        let value = token.literal.to_owned();

        Self {
            token: token.clone(),
            value,
        }
    }
}

impl Expression for Identifier {
    fn eval(&self) -> String {
        todo!()
    }
}

impl Node for Identifier {
    fn get_token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
