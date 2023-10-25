use crate::traits::{expression::Expression, node::Node};

use super::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct TypedIdentifier {
    pub identifier: Identifier,
    pub t: Option<Identifier>,
}

impl TypedIdentifier {
    pub fn new(identifier: Identifier, t: Option<Identifier>) -> Self {
        Self { identifier, t }
    }
}

impl Expression for TypedIdentifier {
    fn eval(&self) -> String {
        self.get_token_literal()
    }

    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for TypedIdentifier {
    fn get_token_literal(&self) -> String {
        self.identifier.get_token_literal()
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

impl ToString for TypedIdentifier {
    fn to_string(&self) -> String {
        if let Some(t) = &self.t {
            format!("{}: {}", self.identifier.value.to_string(), t.to_string())
        } else {
            self.identifier.value.to_string()
        }
    }
}
