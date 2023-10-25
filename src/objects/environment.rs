use std::collections::HashMap;

use crate::traits::object::Object;

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub store: HashMap<String, Box<dyn Object>>,
    pub outer: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(outer: Option<&'a Environment<'a>>) -> Self {
        Self {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn get(&self, name: String) -> Option<&Box<dyn Object>> {
        if let Some(outer_env) = self.outer {
            if let Some(value) = outer_env.get(name.clone()) {
                return Some(value);
            }
        }

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
