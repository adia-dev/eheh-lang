use super::node::Node;

pub trait Expression: Node {
    fn eval(&self) -> String;
    fn clone_boxed(&self) -> Box<dyn Expression>;
}

impl core::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression{{{}}}", self.eval())
    }
}


impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.as_ref().clone_boxed()
    }
}
