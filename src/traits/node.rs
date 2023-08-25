use std::any::Any;

pub trait Node: ToString {
    fn get_token_literal(&self) -> String;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    fn as_node(&self) -> &dyn Node;
}
