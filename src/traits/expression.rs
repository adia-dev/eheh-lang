use super::node::Node;

pub trait Expression: Node {
    fn eval(&self) -> String;
}

impl core::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression{{{}}}", self.eval())
    }
}
