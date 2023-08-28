use std::collections::HashMap;

use crate::traits::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<&Box<dyn Object>> {
        self.store.get(&name)
    }

    pub fn get_mut(&mut self, name: String) -> Option<&mut Box<dyn Object>> {
        self.store.get_mut(&name)
    }

    pub fn set(&mut self, name: String, value: &Box<dyn Object>) -> Option<Box<dyn Object>> {
        self.store.insert(name, value.clone())
    }

    pub fn remove(&mut self, name: String) -> Option<Box<dyn Object>> {
        self.store.remove(&name)
    }

    pub fn has(&mut self, name: String) -> bool {
        self.store.contains_key(&name)
    }
}
