use std::{error::Error, fmt::Display};

use colored::Colorize;

use crate::token::{token_type::TokenType, Token};

#[derive(Debug, Clone)]
#[repr(usize)]
pub enum RuntimeErrorCode {
    VariableNotFound {
        variable_name: String,
        context: Option<String>,
    },
    DivisionByZero {
        location: Token,
    },
    InvalidOperation {
        operation: String,
        context: Option<String>,
    },
    TypeMismatch {
        expected_type: String,
        actual_type: String,
        location: Token,
    },
    TypeNotFound {
        expected_type: String,
        location: Token,
    },
    FunctionNotFound {
        function_name: String,
        context: Option<String>,
    },
    IndexOutOfRange {
        index: isize,
        collection: String,
        location: Token,
    },
    InvalidArraySize {
        size: isize,
        location: Token,
    },
    UnknownInfixOperator {
        operator: String,
        context: Option<String>,
    },
    // structs, fields and modules
    PrivateAccessError,
    InaccessibleModule,
    InaccessibleFunction,
    InaccessibleField,
    InaccessibleEnum,
    // Invalid evaluation of values (const, static, uninitialized)
    ConstEvaluationError,
    InvalidConstValue,
    UninitializedVariable,
    UninitializedStatic,
    // Misc Errors
    OverflowError,
    UnderflowError,
    UnreachableCode,
    UnresolvedName,
    UnexpectedToken,
    Custom(String), // For more specific or custom errors
}

impl RuntimeErrorCode {
    pub fn id(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorCode::VariableNotFound {
                variable_name,
                context,
            } => {
                write!(f, "Variable '{}' not found", variable_name)?;
                if let Some(ctx) = context {
                    writeln!(f, "\n{}", ctx)?;
                }
            }
            RuntimeErrorCode::DivisionByZero { location } => {
                writeln!(f, "Division by zero at {}", location.get_location())?;
            }
            RuntimeErrorCode::InvalidOperation { operation, context } => {
                write!(f, "{}", format!("runtime_error[E{:0>5}]", self.id()).red())?;
                write!(f, ": Invalid operation: {}", operation)?;
                if let Some(ctx) = context {
                    writeln!(f, "\n{}", ctx)?;
                }
            }
            RuntimeErrorCode::TypeMismatch {
                expected_type,
                actual_type,
                location,
            } => {
                writeln!(
                    f,
                    "Type mismatch: Expected {} but got {} at {}",
                    expected_type,
                    actual_type,
                    location.get_location()
                )?;
            }
            RuntimeErrorCode::FunctionNotFound {
                function_name,
                context,
            } => {
                write!(f, "Function '{}' not found", function_name)?;
                if let Some(ctx) = context {
                    writeln!(f, "\n{}", ctx)?;
                }
            }
            RuntimeErrorCode::IndexOutOfRange {
                index,
                collection,
                location,
            } => {
                writeln!(
                    f,
                    "Index {} is out of range for {} at {}",
                    index,
                    collection,
                    location.get_location()
                )?;
            }
            RuntimeErrorCode::InvalidArraySize { size, location } => {
                writeln!(
                    f,
                    "Invalid array size {} at {}",
                    size,
                    location.get_location()
                )?;
            }
            RuntimeErrorCode::UnknownInfixOperator { operator, context } => {
                write!(f, "{}", format!("runtime_error[E{:0>5}]", self.id()).red())?;
                writeln!(f, ": Unknown infix operator: {}", operator)?;

                if let Some(ctx) = context {
                    let operator_pos = ctx.find(operator).unwrap();

                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(f, "{:3} {}\t{}", "10".to_string().blue(), "|".blue(), ctx)?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(operator_pos),
                        "^".repeat(operator.len()).red()
                    )?;
                }
            }
            RuntimeErrorCode::Custom(message) => {
                write!(f, "{}", format!("runtime_error[E{:0>5}]", self.id()).red())?;
                writeln!(f, " {}", message)?;
            }
            err => {
                write!(f, "{}", format!("runtime_error[E{:0>5}]", self.id()).red())?;
                writeln!(f, ": unimplemented error({:?})", self)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub source: Option<Box<RuntimeError>>,
    pub code: RuntimeErrorCode,
}

impl RuntimeError {
    pub fn set_source(&mut self, err: RuntimeError) {
        self.source = Some(Box::new(err));
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Error for RuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(sauce) = &self.source {
            Some(sauce)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_various_errors() {}
}
