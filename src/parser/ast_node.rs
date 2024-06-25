use super::{exprql_parser::*, verbs::*};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi)]
#[serde(tag = "type", content = "content")]
pub enum Literal {
    True,
    False,
    Null,

    Column(String),
    String(String),
    Number(f64),
    List(Vec<AstNode>),
    LookupList(String),
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi)]
#[serde(untagged)]
pub enum AstNode {
    UnaryOp {
        verb: UnaryVerb,
        expr: Box<AstNode>,
        inverted: bool,
    },
    BinaryOp {
        verb: BinaryVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        inverted: bool,
    },
    Literal(Literal),
}

pub fn build_ast_from_expr(pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::Expression => build_ast_from_expr(primary.into_inner()),
            Rule::List => AstNode::Literal(Literal::List(
                primary
                    .into_inner()
                    .map(|pair| build_ast_from_expr(pair.into_inner()))
                    .collect(),
            )),
            Rule::LookupList => AstNode::Literal(Literal::LookupList(primary.as_str().to_owned())),

            Rule::True => AstNode::Literal(Literal::True),
            Rule::False => AstNode::Literal(Literal::False),
            Rule::Null => AstNode::Literal(Literal::Null),
            Rule::Column => AstNode::Literal(Literal::Column(primary.as_str().to_owned())),
            Rule::String => AstNode::Literal(Literal::String(primary.as_str().to_owned())),
            Rule::Number => AstNode::Literal(Literal::Number(
                primary.as_str().parse::<f64>().unwrap_or(0_f64),
            )),

            rule => todo!("unknown rule {:?}", rule),
        })
        .map_infix(|lhs, verb, rhs| parse_binary_verb(verb, lhs, rhs))
        .map_prefix(|verb, expr| parse_unary_verb(verb, expr))
        .map_postfix(|expr, verb| parse_unary_verb(verb, expr))
        .parse(pairs)
}
