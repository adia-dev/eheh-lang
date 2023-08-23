use std::any::Any;

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
    Null,
}

pub trait Object: ToString {
    /// type of the object -> `ObjectType`
    fn t(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
