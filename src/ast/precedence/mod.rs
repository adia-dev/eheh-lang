use std::fmt::Display;

use crate::token::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST = -1,
    EQ,
    LGT,
    BITWISE,
    SUM,
    PRODUCT,
    EXPONENT,
    PREFIX,
    CALL,
}

impl Precedence {
    pub fn from_token_type(t: &TokenType) -> Precedence {
        match t {
            TokenType::EQ | TokenType::NEQ => Precedence::EQ,
            TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE => Precedence::LGT,
            TokenType::LSHIFT | TokenType::RSHIFT => Precedence::BITWISE,
            TokenType::PLUS | TokenType::MINUS => Precedence::SUM,
            TokenType::ASTERISK | TokenType::FORWARDSLASH | TokenType::PERCENT => {
                Precedence::PRODUCT
            }
            TokenType::EXPONENT => Precedence::EXPONENT,
            TokenType::INCR | TokenType::DECR | TokenType::BANG => Precedence::PREFIX,
            _ => Precedence::LOWEST,
        }
    }
}

impl Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

