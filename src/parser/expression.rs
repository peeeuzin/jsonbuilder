use pest::iterators::Pair;

use super::{Expression, Rule};

pub fn parse(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::ident => Expression::Identifier(pair.as_str().to_string()),
        Rule::namespace => {
            let namespaces = pair.into_inner().map(|p| p.as_str().to_string()).collect();
            Expression::Namespace(namespaces)
        }
        Rule::crnt => Expression::Current,
        _ => unreachable!("Unexpected rule: {:?}", pair),
    }
}
