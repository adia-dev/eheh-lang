use crate::{
    ast::{
        expressions::typed_identifier::TypedIdentifier, statements::block_statement::BlockStatement,
    },
    traits::object::{Object, ObjectType},
};

use super::environment::Environment;

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub parameters: Vec<TypedIdentifier>,
    pub body: BlockStatement,
    pub env: Environment<'a>,
}

impl<'a> Function<'a> {}

impl<'a> Object for Function<'static> {
    fn t(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    fn clone_boxed(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl<'a> ToString for Function<'a> {
    fn to_string(&self) -> String {
        "".to_owned()
    }
}
