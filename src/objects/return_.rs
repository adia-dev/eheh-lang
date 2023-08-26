use crate::traits::object::{Object, ObjectType};

#[derive(Debug, Clone)]
pub struct Return {
    pub value: Option<Box<dyn Object>>,
}

impl Return {
    pub fn new(value: Option<Box<dyn Object>>) -> Self {
        Self { value }
    }
}

impl Object for Return {
    fn t(&self) -> ObjectType {
        ObjectType::Return
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


impl ToString for Return {
    fn to_string(&self) -> String {
        if let Some(value) = &self.value {
            format!("return {}", value.to_string())
        } else {
            "return".to_string()
        }
    }
}
