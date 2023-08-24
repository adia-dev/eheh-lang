use crate::traits::object::{Object, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null {}

impl Null {
    pub const fn new() -> Self { Self {  } }
}

impl Object for Null {
    fn t(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for Null {
    fn to_string(&self) -> String {
        "null".to_string()
    }
}
