use std::{
    fmt::{self, Debug},
    io,
};

use crate::{builder::error::BuilderError, parser::error::SyntaxError};
pub type Result<T> = std::result::Result<T, JsonBuilderError>;

pub enum JsonBuilderError {
    IO(io::Error),
    SyntaxError(SyntaxError),
    BuilderError(BuilderError),
}

impl Debug for JsonBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonBuilderError::IO(e) => write!(f, "IO error: {}", e),
            JsonBuilderError::SyntaxError(e) => write!(f, "Syntax error: \n{}", e.0),
            JsonBuilderError::BuilderError(e) => write!(f, "Builder error: {}", e.0),
        }
    }
}
