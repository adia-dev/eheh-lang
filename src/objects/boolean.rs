use crate::traits::object::{Object, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub const fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Object for Boolean {
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

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
