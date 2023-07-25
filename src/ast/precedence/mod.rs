#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST = -1,
    EQ,
    LGT,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}
