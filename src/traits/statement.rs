use super::node::Node;

pub trait Statement: Node {
    fn process(&self);
}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement{{{}}}", self.to_string())
    }
}
