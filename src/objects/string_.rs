use crate::traits::object::{IntegerType, Object, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringObj {
    pub content: String,
}

impl StringObj {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
        }
    }
}

impl Object for StringObj {
    fn t(&self) -> ObjectType {
        ObjectType::Integer(IntegerType::I64)
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    fn clone_boxed(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl ToString for StringObj {
    fn to_string(&self) -> String {
        format!("\"{}\"", self.content)
    }
}
