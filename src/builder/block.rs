use serde_json::Value;

use crate::{context::Context, error::*, parser::Block as BlockAST};

use super::statement;

pub fn run(block: BlockAST, context: &Context, value: &mut Value) -> Result<()> {
    for statement in block.0 {
        statement::run(statement, context, value)?;
    }

    Ok(())
}
