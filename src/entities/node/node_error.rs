use std::fmt::{self, Display, Formatter};
use thiserror::Error;

use crate::entities::factories::factory::Factory;

#[derive(Debug, Error, PartialEq)]
#[allow(dead_code)]
pub enum NodeFactoryAddError {
    DuplicateIdWithFactory((u64, Factory)),
    DuplicateNameWithFactory((String, Factory)),
    LimitReachedWithFactory(Factory),
    DuplicateId(u64),
    DuplicateName(String),
    LimitReached(u32),
}

impl Display for NodeFactoryAddError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeFactoryAddError::DuplicateIdWithFactory((id, _)) => {
                write!(f, "DuplicateIdWithFactory: Id is {:?}", id)
            }
            NodeFactoryAddError::DuplicateNameWithFactory((name, _)) => {
                write!(f, "DuplicateNameWithFactory: Name is {:?}", name)
            }
            NodeFactoryAddError::LimitReachedWithFactory(_) => {
                write!(f, "Node contains too many items")
            }
            NodeFactoryAddError::DuplicateId(id) => {
                write!(f, "DuplicateId. Id is {}", id)
            }
            NodeFactoryAddError::DuplicateName(name) => {
                write!(f, "DuplicateName. Name is {}", name)
            }
            NodeFactoryAddError::LimitReached(limit) => {
                write!(f, "LimitReached. Limit is {}", limit)
            }
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum NodeRemoveFactoryError {
    FactoryDoesNotExist(u64),
}

impl Display for NodeRemoveFactoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeRemoveFactoryError::FactoryDoesNotExist(val) => {
                    format!("FactoryDoesNotExist: {}", val)
                }
            }
        )
    }
}
