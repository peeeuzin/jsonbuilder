use super::{statement, Block, Rule};
use crate::error::*;
use pest::iterators::Pairs;

pub fn parse(pairs: Pairs<Rule>) -> Result<Block> {
    let mut statements = Vec::new();

    for pair in pairs {
        statements.push(statement::parse(pair)?);
    }

    Ok(Block(statements))
}
