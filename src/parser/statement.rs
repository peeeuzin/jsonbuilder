use pest::iterators::Pair;

use super::{block, expression, Rule, Statement};
use crate::error::*;

pub fn parse(pair: Pair<Rule>) -> Result<Statement> {
    match pair.as_rule() {
        Rule::decl => {
            let mut inner_rules = pair.into_inner();

            let left = expression::parse(inner_rules.next().unwrap());
            let right = expression::parse(inner_rules.next().unwrap().into_inner().next().unwrap());

            Ok(Statement::Declare { left, right })
        }

        Rule::object => {
            let mut inner_rules = pair.into_inner();

            let left = expression::parse(inner_rules.next().unwrap());
            let block = block::parse(inner_rules.next().unwrap().into_inner())?;

            Ok(Statement::Object { left, block })
        }

        Rule::array_map => {
            let mut inner_rules = pair.into_inner();

            let left = expression::parse(inner_rules.next().unwrap());
            let placeholder =
                expression::parse(inner_rules.next().unwrap().into_inner().next().unwrap());
            let arg = expression::parse(inner_rules.next().unwrap());
            let block = block::parse(inner_rules.next().unwrap().into_inner())?;

            Ok(Statement::ArrayMap {
                left,
                placeholder,
                arg,
                block,
            })
        }

        Rule::statements => parse(pair.into_inner().next().unwrap()),
        _ => unreachable!("Unexpected rule: {:?}", pair),
    }
}
