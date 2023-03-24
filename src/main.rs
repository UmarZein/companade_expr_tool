use companade_expr_tool::*;
use pest::{iterators::*, Parser};

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

    #[test]
    fn multiple() {
        let mut x: pest::iterators::Pairs<Rule> = FCParser::parse(
            Rule::factories,
            r#"
lemonade_maker_o2@100$102_000(plastic_cup: 4, ice_cube: 4, lemon_fruit: 3) 
    => 1: lemonade {sweetness: max(0, norm(0.5,0.2)), coolness: norm(0.8,0.1)}

lemonade_maker_o2@100$100_000(plastic_cup: 4, lemon_fruit: 3) 
    => 1: lemonade {sweetness: max(0, norm(0.5,0.2)), coolness: norm(0.3,0.1)}

ceramic_mixture_factory@30$100_000(talcum: 20) 
    -> 10: ceramic_mixture

tile_press@100$100_000(ceramic_mixture: 5) 
    => 5: ceramic_tile {hardness: norm(20, 1.5), 
                     scratch_resistance: skewnorm(10,5,20)}
"#,
        )
        .unwrap();

        let tmp = x.next().unwrap();

        let y = parse_config::parse_configs(tmp).unwrap();

        println!("{y:#?}");
    }
}

fn main() {
    let mut x: pest::iterators::Pairs<Rule> = FCParser::parse(
        Rule::factories,
        r#"
lemonade_maker_o2@100$102_000(plastic_cup: 4, ice_cube: 4, lemon_fruit: 3) 
    => 1: lemonade {sweetness: max(0, norm(0.5,0.2)), coolness: norm(0.8,0.1)}

lemonade_maker_o2@100$100_000(plastic_cup: 4, lemon_fruit: 3) 
    => 1: lemonade {sweetness: max(0, norm(0.5,0.2)), coolness: norm(0.3,0.1)}

ceramic_mixture_factory@30$100_000(talcum: 20) 
    -> 10: ceramic_mixture

tile_press@100$100_000(ceramic_mixture: 5) 
    => 5: ceramic_tile {hardness: norm(20, 1.5), 
                     scratch_resistance: skewnorm(10,5,20)}
"#,
    )
    .unwrap();

    let tmp = x.next().unwrap();

    let y = parse_config::parse_configs(tmp).unwrap();

    println!("{y:#?}");
}
