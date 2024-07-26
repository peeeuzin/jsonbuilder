use serde_json::Value;

use super::error::*;
use crate::{context::Context, error::*, parser::Expression as ExpressionAST};

pub fn evaluate(expression: ExpressionAST, context: &Context) -> Result<Value> {
    match expression {
        ExpressionAST::Identifier(identifier) => Ok(context.get(&identifier).unwrap().clone()),
        ExpressionAST::Namespace(namespace) => {
            let value = context.get(&namespace[0]);

            if value.is_none() {
                return Err(JsonBuilderError::BuilderError(BuilderError::from_error(
                    BuilderErrorKind::VariableNotFound(namespace[0].clone()),
                )));
            }

            let mut value = value.unwrap().clone();

            for key in namespace.iter().skip(1) {
                value = value[key].clone();
            }

            Ok(value)
        }
        ExpressionAST::Current => Ok(Value::Null),
    }
}
