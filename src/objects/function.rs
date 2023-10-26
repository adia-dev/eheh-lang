use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expressions::{identifier::Identifier, typed_identifier::TypedIdentifier},
        statements::block_statement::BlockStatement,
    },
    traits::{
        node::Node,
        object::{Object, ObjectType},
    },
};

use super::environment::Environment;

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<TypedIdentifier>,
    pub body: BlockStatement,
    pub return_type: Option<Identifier>,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<TypedIdentifier>,
        body: BlockStatement,
        return_type: Option<Identifier>,
        env: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            parameters,
            body,
            return_type,
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
        let mut str = String::new();
        str.push_str("fn ");

        str.push_str("(");

        self.parameters.iter().enumerate().for_each(|(i, param)| {
            str.push_str(param.to_string().as_str());
            if i < self.parameters.len() - 1 {
                str.push_str(", ");
            }
        });

        str.push_str(")");

        if let Some(return_type) = &self.return_type {
            str.push_str(format!(" -> {} ", return_type.get_token_literal()).as_str());
        }

        str.push_str(self.body.to_string().as_str());

        str
    }
}
