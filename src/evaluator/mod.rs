use crate::{traits::{object::Object, node::Node}, types::EvaluatorResult};


#[derive(Debug, Clone)]
pub struct Evaluator {
}

impl Evaluator {
    pub fn eval(node: Box<dyn Node>) -> EvaluatorResult {
        Err("eheh".into())
    }
}

#[cfg(test)]
pub mod test;
