use std::fmt;

#[derive(Debug)]
pub struct BuilderError(pub String);

impl BuilderError {
    pub fn from_error(error: BuilderErrorKind) -> Self {
        Self(error.to_string())
    }
}

pub enum BuilderErrorKind {
    VariableNotFound(String),
    InvalidType(String),
}

impl fmt::Display for BuilderErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuilderErrorKind::VariableNotFound(variable) => {
                write!(f, "Variable not found: {}", variable)
            }
            BuilderErrorKind::InvalidType(variable) => {
                write!(f, "Invalid type: {}", variable)
            }
        }
    }
}
