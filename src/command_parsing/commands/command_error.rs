use std::num::ParseIntError;

use crate::entities::node::node_error::{NodeFactoryAddError, NodeRemoveFactoryError};

#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandError {
    MissingArgument(String),
    CommandNodeFactoryAddError(NodeFactoryAddError),
    CommandParseIntError(ParseIntError),
    CommandNodeFactoryRemoveError(NodeRemoveFactoryError),
}
