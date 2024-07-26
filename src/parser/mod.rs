use error::SyntaxError;
use pest::Parser;

pub mod block;
pub mod error;
pub mod expression;
pub mod statement;

use crate::error::{JsonBuilderError, Result};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/jsonbuilder.pest"]
struct JsonBuilderParser;

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, Clone)]
pub enum Statement {
    Declare {
        left: Expression,
        right: Expression,
    },
    Object {
        left: Expression,
        block: Block,
    },
    ArrayMap {
        left: Expression,
        placeholder: Expression,
        arg: Expression,
        block: Block,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Namespace(Vec<String>),
    Current,
}

pub fn parse(input: &str) -> Result<Block> {
    let mut pairs = match JsonBuilderParser::parse(Rule::program, input) {
        Ok(pairs) => pairs,
        Err(e) => return Err(JsonBuilderError::SyntaxError(SyntaxError::from_error(e))),
    };

    block::parse(pairs.next().unwrap().into_inner())
}
