use crate::traits::object::{IntegerType, Object, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Object for Integer {
    fn t(&self) -> ObjectType {
        ObjectType::Integer(IntegerType::I64)
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
