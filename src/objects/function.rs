use std::{rc::Rc, cell::RefCell};

use crate::{
    ast::{
        expressions::typed_identifier::TypedIdentifier, statements::block_statement::BlockStatement,
    },
    traits::object::{Object, ObjectType},
};

use super::environment::Environment;

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<TypedIdentifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<TypedIdentifier>,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
}

impl Object for Function {
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

impl ToString for Function {
    fn to_string(&self) -> String {
        "".to_owned()
    }
}
