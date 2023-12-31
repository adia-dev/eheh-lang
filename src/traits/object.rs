use std::{any::Any, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegerType {
    // Signed
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    IMin,
    IMax,

    // Unsigned
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    UMin,
    UMax,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectType {
    Boolean,
    Integer(IntegerType),
    Return,
    Function,
    Error,
    Null,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Boolean => write!(f, "boolean"),
            ObjectType::Return => write!(f, "return"),
            ObjectType::Error => write!(f, "error"),
            ObjectType::Function => write!(f, "function"),
            ObjectType::Null => write!(f, "null"),
            ObjectType::Integer(i) => i.fmt(f),
        }
    }
}

impl Display for IntegerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerType::I8 => write!(f, "i8"),
            IntegerType::I16 => write!(f, "i16"),
            IntegerType::I32 => write!(f, "i32"),
            IntegerType::I64 => write!(f, "i64"),
            IntegerType::I128 => write!(f, "i128"),
            IntegerType::ISize => write!(f, "isize"),
            IntegerType::IMin => write!(f, "imin"),
            IntegerType::IMax => write!(f, "imax"),
            IntegerType::U8 => write!(f, "u8"),
            IntegerType::U16 => write!(f, "u16"),
            IntegerType::U32 => write!(f, "u32"),
            IntegerType::U64 => write!(f, "u64"),
            IntegerType::U128 => write!(f, "u128"),
            IntegerType::USize => write!(f, "usize"),
            IntegerType::UMin => write!(f, "umin"),
            IntegerType::UMax => write!(f, "umax"),
        }
    }
}

pub trait Object: ToString {
    /// type of the object -> `ObjectType`
    fn t(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(self) -> Box<dyn Any>;
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_boxed(&self) -> Box<dyn Object>;
}

impl core::fmt::Debug for dyn Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object{{{}}}", self.to_string())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.as_ref().clone_boxed()
    }
}
