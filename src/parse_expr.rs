use crate::*;
/// #Panics
///
/// just know that it can panic. UNDOCUMENTED
pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    let res = PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Atom(primary.as_str().replace("_", "").parse::<f64>().unwrap()),
            Rule::expr => parse_expr(pairs.clone().next().unwrap().into_inner()),
            Rule::clip => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let val = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                let min = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                let max = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                Expr::Clip { val, min, max }
            }
            Rule::min => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let a = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                let b = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                Expr::Min { a, b }
            }
            Rule::max => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let a = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                let b = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                Expr::Max { a, b }
            }
            Rule::norm => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let mean = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                let std = Box::new(parse_expr(pairs.next().unwrap().into_inner()));
                Expr::Norm { mean, std }
            }
            Rule::skwn => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let loc = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                let scale = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                let shape = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Skewnorm { loc, scale, shape }
            }
            Rule::logn => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let base = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                let arg = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Logn { base, arg }
            }
            Rule::log2 => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let arg = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Log2 { arg }
            }
            Rule::ln => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let arg = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Ln { arg }
            }
            Rule::abs => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let val = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Abs { val }
            }
            Rule::sqrt => {
                let mut pairs = pairs.clone();
                let mut pairs = pairs.next().unwrap().into_inner();
                let val = Box::new(parse_expr(
                    pairs
                        .next()
                        .ok_or(ParseError::Placeholder)
                        .unwrap()
                        .into_inner(),
                ));
                Expr::Sqrt { val }
            }
            Rule::neg => Expr::Neg {
                val: Box::new(parse_expr(pairs.clone().next().unwrap().into_inner())),
            },
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Expr::Add {
                a: Box::new(lhs),
                b: Box::new(rhs),
            },
            Rule::subtract => Expr::Sub {
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::multiply => Expr::Mul {
                a: Box::new(lhs),
                b: Box::new(rhs),
            },
            Rule::divide => Expr::Div {
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::power => Expr::Pow {
                base: Box::new(lhs),
                exp: Box::new(rhs),
            },
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .parse(pairs.clone());
    return res;
}
