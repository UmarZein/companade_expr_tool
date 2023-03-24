extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "./rule.pest"]
/// FCParser = Factory Config Parser
pub struct FCParser;
pub struct TEST {}
use pest::{iterators::*, pratt_parser::PrattParser};

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(power, Left))
    };
}

pub mod eval_expr;
pub mod expr;
pub mod factory_config;
pub mod parse_config;
pub mod parse_expr;
pub use eval_expr::*;
pub use expr::*;
pub use factory_config::*;
pub use parse_config::*;
pub use parse_expr::*;
