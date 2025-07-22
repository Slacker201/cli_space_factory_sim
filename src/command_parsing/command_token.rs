use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
/// This is a simple type for command line arguments
pub enum CommandToken {
    BooleanTrue,
    Value(String),
}
/// Impl block for ArgumentFlag
impl Display for CommandToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            CommandToken::BooleanTrue => "True",
            CommandToken::Value(a) => a,
        };
        write!(f, "{val}")
    }
}
