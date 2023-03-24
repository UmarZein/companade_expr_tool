use pest::{iterators::*, Parser};
use companade_expr_tool::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let x: pest::iterators::Pairs<Rule> = FCParser::parse(
            Rule::factory,
            r#"lemonade_maker_x22 @ 86_400 $1_000_000 (
        plastic_cup: 10, 
        metal_straw: 5, 
        drinking_water: 30, 
        palachian_lemon_fruit: 50)
        => 1: lemonade {
            sweetness: clip(norm(0.32,3),0,1),
            volume: max(norm(30,2),0),
            density: 1000,
            viscosity:0.9
        }
    "#,
        )
        .unwrap();
        let tmp: Vec<Pair<_>> = x.collect();

        let y = parse_config::parse_config(tmp[0].clone()).unwrap();

        println!("{y:#?}");
    }
}

fn main() {
    let x: pest::iterators::Pairs<Rule> = FCParser::parse(
        Rule::factory,
        r#"lemonade_maker_x22 @ 86_400 $1_000_000 (
    plastic_cup: 10, 
    metal_straw: 5, 
    drinking_water: 30, 
    palachian_lemon_fruit: 50)
    => 1: lemonade {
        sweetness: clip(norm(0.32,3),0,1),
        volume: max(norm(30,2),0),
        density: 1000,
        viscosity: 0.9
    }
"#,
    )
    .unwrap();
    let tmp: Vec<Pair<_>> = x.collect();

    let y = parse_config::parse_config(tmp[0].clone()).unwrap();

    println!("{y:#?}");
}
