#![allow(non_snake_case)] // bad code gen inside Tsify derive

use ast_node::*;
use cfg_if::cfg_if;
use exprql_parser::*;
use pest::Parser;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
    }
}

pub mod ast_node;
pub mod exprql_parser;
pub mod verbs;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi)]
pub struct ParseResult {
    ast: Vec<AstNode>,
}

#[wasm_bindgen]
pub fn parse(source: &str) -> Result<ParseResult, String> {
    cfg_if! {
        if #[cfg(feature = "console_error_panic_hook")] {
            console_error_panic_hook::set_once();
        }
    }

    let mut ast = vec![];

    let pairs = match ExprQLParser::parse(Rule::Program, source) {
        Ok(pairs) => pairs,
        Err(err) => return Err(err.to_string()),
    };

    for pair in pairs.clone() {
        if pair.as_rule() == Rule::Expression {
            ast.push(build_ast_from_expr(pair.into_inner()));
        }
    }

    Ok(ParseResult { ast })
}
