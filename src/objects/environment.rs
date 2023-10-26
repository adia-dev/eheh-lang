use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::traits::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Rc<RefCell<Box<dyn Object>>>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Box<dyn Object>>>> {
        if let Some(outer_env) = &self.outer {
            if let Some(value) = outer_env.borrow().store.get(name) {
                return Some(value.clone());
            }
        }
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Box<dyn Object>) -> Option<Rc<RefCell<Box<dyn Object>>>> {
        self.store.insert(name, Rc::new(RefCell::new(value)))
    }

    pub fn remove(&mut self, name: &str) -> Option<Rc<RefCell<Box<dyn Object>>>> {
        self.store.remove(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }
}

