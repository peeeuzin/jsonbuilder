use crate::{
    context::Context,
    error::*,
    parser::{Expression as ExpressionAST, Statement as StatementAST},
};
use serde_json::{Map, Value};

use super::{
    block,
    error::{BuilderError, BuilderErrorKind},
    expression,
};

pub fn run(ast: StatementAST, context: &Context, value: &mut Value) -> Result<()> {
    match ast {
        StatementAST::Declare { left, right } => {
            let right = expression::evaluate(right, context)?;

            let left = match left {
                ExpressionAST::Identifier(identifier) => vec![identifier],
                ExpressionAST::Namespace(namespace) => namespace,
                ExpressionAST::Current => {
                    *value = right;

                    return Ok(());
                }
            };

            insert(value, left, right);
        }

        StatementAST::ArrayMap {
            left,
            placeholder,
            arg,
            block,
        } => {
            let left = match left {
                ExpressionAST::Identifier(identifier) => vec![identifier],
                ExpressionAST::Namespace(namespace) => namespace,
                ExpressionAST::Current => {
                    return Err(JsonBuilderError::BuilderError(BuilderError::from_error(
                        BuilderErrorKind::InvalidType(format!("{:?}", left)),
                    )))
                }
            };

            let arg = match arg {
                ExpressionAST::Identifier(identifier) => identifier,
                _ => {
                    return Err(JsonBuilderError::BuilderError(BuilderError::from_error(
                        BuilderErrorKind::InvalidType(format!("{:?}", arg)),
                    )))
                }
            };

            let placeholder = match expression::evaluate(placeholder.clone(), context)? {
                Value::Array(array) => array,
                _ => {
                    return Err(JsonBuilderError::BuilderError(BuilderError::from_error(
                        BuilderErrorKind::InvalidType(format!("{:?}", placeholder)),
                    )))
                }
            };

            let mut array = Vec::new();

            for array_values in placeholder {
                let mut context = context.clone();
                context.insert(arg.clone(), array_values);

                let mut block_value = Value::Object(Map::new());

                block::run(block.clone(), &context, &mut block_value)?;

                array.push(block_value);
            }

            insert(value, left, Value::Array(array));
        }

        StatementAST::Object { left, block } => {
            let left = match left {
                ExpressionAST::Identifier(identifier) => vec![identifier],
                ExpressionAST::Namespace(namespace) => namespace,
                ExpressionAST::Current => {
                    return Err(JsonBuilderError::BuilderError(BuilderError::from_error(
                        BuilderErrorKind::InvalidType(format!("{:?}", left)),
                    )))
                }
            };

            let mut block_value = Value::Object(Map::new());

            block::run(block, context, &mut block_value)?;

            insert(value, left, block_value);
        }
    }

    Ok(())
}

fn insert(value: &mut Value, mut keys: Vec<String>, right: Value) {
    let mut current = value;
    let last = keys.pop().unwrap();

    for key in keys.iter() {
        let entry = current
            .as_object_mut()
            .unwrap()
            .entry(key.clone())
            .or_insert_with(|| Value::Object(Map::new()));

        current = entry;
    }

    current.as_object_mut().unwrap().insert(last, right);
}
