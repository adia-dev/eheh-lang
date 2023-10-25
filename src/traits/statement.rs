use super::node::Node;

pub trait Statement: Node {
    fn process(&self);
    fn clone_boxed(&self) -> Box<dyn Statement>;
}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement{{{}}}", self.to_string())
    }
}


impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        self.as_ref().clone_boxed()
    }
}
