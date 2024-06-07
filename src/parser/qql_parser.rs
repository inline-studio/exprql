use lazy_static::lazy_static;
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
pub struct QQLParser;

lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(Or, Left))
            .op(Op::infix(Between, Left))
            .op(Op::infix(And, Left))
            .op(Op::prefix(UnaryNot))
            .op(Op::infix(Eq, Right)
                | Op::infix(NotEq, Right)
                | Op::infix(NotEq, Right)
                | Op::infix(Gt, Right)
                | Op::infix(GtEq, Right)
                | Op::infix(Lt, Right)
                | Op::infix(LtEq, Right)
                | Op::infix(In, Right)
                | Op::infix(Like, Right)
                | Op::infix(Regexp, Right))
            .op(Op::infix(Add, Left) | Op::infix(Subtract, Left))
            .op(Op::infix(Multiply, Left) | Op::infix(Divide, Left) | Op::infix(Concat, Left))
            .op(Op::postfix(IsNullPostfix))
    };
}
