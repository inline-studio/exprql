use super::{ast_node::AstNode, exprql_parser::Rule};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi)]
pub enum UnaryVerb {
    Not,
    IsNull,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi)]
pub enum BinaryVerb {
    And,
    Or,

    Between,
    Concat,

    Add,
    Subtract,
    Multiply,
    Divide,
    Eq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    NotEq,

    In,
    Like,
    Regexp,
}

pub fn parse_unary_verb(pair: pest::iterators::Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::UnaryOp {
        verb: match pair.as_rule() {
            Rule::NotFlag => UnaryVerb::Not,
            Rule::IsNullPostfix => UnaryVerb::IsNull,
            _ => panic!("unknown unary verb: {}", pair.as_str()),
        },
        expr: Box::new(expr),
    }
}

pub fn parse_binary_verb(pair: pest::iterators::Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
    AstNode::BinaryOp {
        verb: match pair.as_rule() {
            Rule::And => BinaryVerb::And,
            Rule::Or => BinaryVerb::Or,

            Rule::Between => BinaryVerb::Between,
            Rule::Concat => BinaryVerb::Concat,

            Rule::Add => BinaryVerb::Add,
            Rule::Subtract => BinaryVerb::Subtract,
            Rule::Multiply => BinaryVerb::Multiply,
            Rule::Divide => BinaryVerb::Divide,
            Rule::Eq => BinaryVerb::Eq,
            Rule::Gt => BinaryVerb::Gt,
            Rule::GtEq => BinaryVerb::GtEq,
            Rule::Lt => BinaryVerb::Lt,
            Rule::LtEq => BinaryVerb::LtEq,
            Rule::NotEq => BinaryVerb::NotEq,

            Rule::In => BinaryVerb::In,
            Rule::Like => BinaryVerb::Like,
            Rule::Regexp => BinaryVerb::Regexp,

            _ => panic!("unknown binary verb: {}", pair.as_str()),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}
