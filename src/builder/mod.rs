use crate::{context::Context, error::*, parser::Block};
use serde_json::{Map, Value};

pub mod block;
pub mod error;
pub mod expression;
pub mod statement;

pub fn build(ast: Block, context: Context) -> Result<Value> {
    let mut value = Value::Object(Map::new());

    block::run(ast, &context, &mut value)?;

    Ok(value)
}
