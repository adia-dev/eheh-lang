use crate::traits::object::{IntegerType, Object, ObjectType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    value: i64,
}

impl Object for Integer {
    fn t(&self) -> ObjectType {
        ObjectType::Integer(IntegerType::I64)
    }

    fn inspect(&self) -> String {
        self.to_string()
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
