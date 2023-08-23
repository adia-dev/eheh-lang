use std::{collections::VecDeque, error::Error, fmt::Display};

use colored::Colorize;

use crate::token::{token_type::TokenType, Token};

#[derive(Debug)]
#[repr(usize)]
pub enum ParserWarning {
    UnnecessaryParentheses {
        token: Token,
        context: Vec<String>,
    } = 1_000,
    EmptyIfExpression {
        token: Token,
        context: Vec<String>,
    },
    EmptyIfConsequenceBranch {
        token: Token,
        context: Vec<String>,
        has_alternative: bool,
    },
    EmptyIfAlternativeBranch {
        token: Token,
        context: Vec<String>,
    },
    RedundantIfBranch {
        token: Token,
        context: Vec<String>,
        branch: bool,
    },
    PredictableIfBranch {
        token: Token,
        context: Vec<String>,
        is_true_branch: bool,
    },
    EmptyFunction {
        token: Token,
        context: Vec<String>,
    },
    Unknown,
}

impl ParserWarning {
    fn id(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

impl Display for ParserWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserWarning::UnnecessaryParentheses { token, context } => {
                writeln!(
                    f,
                    "{}: Unnecessary parentheses around {}",
                    format!("warning[E{:0>5}]", self.id()).yellow(),
                    token.literal
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::Unknown => todo!(),
            ParserWarning::EmptyIfExpression { token, context } => {
                writeln!(
                    f,
                    "{}: The if expression has no meaningful consequence or alternative. Consider removing it.",
                    format!("warning[E{:0>5}]", self.id()).yellow()
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::EmptyIfConsequenceBranch {
                token,
                context,
                has_alternative,
            } => {
                if *has_alternative {
                    writeln!(
                        f,
                        "{}: The consequence of the if expression is empty. Consider swapping the alternative and consequence branches of the if expression.",
                        format!("warning[E{:0>5}]", self.id()).yellow()
                        )?;
                } else {
                    writeln!(
                        f,
                        "{}: The if expression has no meaningful consequence. Consider removing it.",
                        format!("warning[E{:0>5}]", self.id()).yellow()
                        )?;
                }

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::EmptyIfAlternativeBranch { token, context } => {
                writeln!(
                    f,
                    "{}: The alternative of the if expression is empty, consider removing the alternative branch.",
                    format!("warning[E{:0>5}]", self.id()).yellow()
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::EmptyFunction { token, context } => {
                writeln!(
                    f,
                    "{}: The function has an empty body. Consider removing it if unnecessary.",
                    format!("warning[E{:0>5}]", self.id()).yellow()
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
            ParserWarning::RedundantIfBranch {
                token,
                context,
                branch,
            } => todo!(),
            ParserWarning::PredictableIfBranch {
                token,
                context,
                is_true_branch,
            } => {
                writeln!(
                    f,
                    "{}: The if expression always selects the {} branch, consider removing the other branch and inlining the if expression.",
                    format!("warning[E{:0>5}]", self.id()).yellow(),
                    if *is_true_branch {"true"} else {"false"}
                )?;

                if !context.is_empty() {
                    writeln!(f, "    {}", "|".blue())?;
                    writeln!(
                        f,
                        "{:3} {}\t{}",
                        token.line.to_string().blue(),
                        "|".blue(),
                        context.join(format!("\n{:3} {}", " ", "|".blue()).as_str())
                    )?;
                    write!(
                        f,
                        "    {}\t{}{} ",
                        "|".blue(),
                        " ".repeat(token.column - 1),
                        "^".repeat(token.literal.len()).yellow()
                    )?;
                }
                writeln!(f, "")
            }
        }
    }
}
