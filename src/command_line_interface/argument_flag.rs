use std::fmt::{self, Display};

#[derive(Debug)]
/// This is a simple type for command line arguments
pub enum ArgumentFlag {
    BooleanTrue,
    Value(String),
}
/// Impl block for ArgumentFlag
impl Display for ArgumentFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            ArgumentFlag::BooleanTrue => "True",
            ArgumentFlag::Value(a) => a,
        };
        write!(f, "{val}")
    }
}