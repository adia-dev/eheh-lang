pub trait Node: ToString {
    fn get_token_literal(&self) -> String;
}
