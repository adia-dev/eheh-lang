use std::sync::Arc;

use crate::traits::object::{Object, ObjectType};

#[derive(Debug, Clone)]
pub struct Error {
    pub err: Arc<dyn std::error::Error>,
}

impl Error {
    pub fn new(err: Arc<dyn std::error::Error>) -> Self {
        Self { err }
    }
}

impl Object for Error {
    fn t(&self) -> ObjectType {
        ObjectType::Error
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

impl ToString for Error {
    fn to_string(&self) -> String {
        self.err.to_string()
    }
}
