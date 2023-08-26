use std::fmt::Display;

use crate::token::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST = -1,
    RANGE,
    OR,
    AND,
    EQ,
    LGT,
    BITWISE,
    SUM,
    PRODUCT,
    EXPONENT,
    CAST,
    PREFIX,
    FieldAccess,
    CALL,
    PATH
}

impl Precedence {
    pub fn from_token_type(t: &TokenType) -> Self {
        match t {
            TokenType::EQ | TokenType::NEQ => Self::EQ,
            TokenType::AND => Self::AND,
            TokenType::OR => Self::OR,
            TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE => Self::LGT,
            TokenType::LSHIFT | TokenType::RSHIFT => Self::BITWISE,
            TokenType::IRANGE | TokenType::RANGE => Self::RANGE,
            TokenType::PLUS | TokenType::MINUS => Self::SUM,
            TokenType::ASTERISK | TokenType::FORWARDSLASH | TokenType::PERCENT => Self::PRODUCT,
            TokenType::EXPONENT | TokenType::DASTERISK => Self::EXPONENT,
            TokenType::INCR | TokenType::DECR | TokenType::BANG => Self::PREFIX,
            TokenType::LPAREN => Self::CALL,
            _ => Self::LOWEST,
        }
    }
}

impl Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
