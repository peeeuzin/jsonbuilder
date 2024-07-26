use pest::{error, iterators::Pair};

use super::Rule;

#[derive(Debug)]
pub struct SyntaxError(pub String);

impl SyntaxError {
    pub fn new(msg: &str, pair: Pair<Rule>) -> Self {
        let error_variant: error::ErrorVariant<Rule> = error::ErrorVariant::CustomError {
            message: msg.to_string(),
        };

        let grammar_error = error::Error::new_from_span(error_variant, pair.as_span());

        Self(grammar_error.to_string())
    }

    pub fn from_error(error: error::Error<Rule>) -> Self {
        Self(error.to_string())
    }
}
