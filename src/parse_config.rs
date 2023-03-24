use crate::*;
use std::collections::hash_map::HashMap;

#[derive(Debug, Clone)]
pub enum ParseError {
    Placeholder,
}

pub fn parse_configs(pairs: Pair<Rule>) -> Result<Vec<FactoryConfig>,ParseError> {
    match &pairs.as_rule(){
        Rule::factories => {
            let tmp = pairs.into_inner().map(|x|parse_config(x));
            let mut buff = vec![];
            for i in tmp{
                buff.push(i?);
            }
            return Ok(buff)
        },
        _ => Err(ParseError::Placeholder)
    }
}

pub fn parse_config(pairs: Pair<Rule>) -> Result<FactoryConfig, ParseError> {
    //let mut res = FactoryConfig::blank_item();
    let mut inner = pairs.clone().into_inner().into_iter();

    let name = inner.next().ok_or(ParseError::Placeholder)?;
    if name.as_rule() != Rule::ident {
        return Err(ParseError::Placeholder);
    }
    let name = name.as_str().to_string();

    let game_daily_frequency = inner.next().ok_or(ParseError::Placeholder)?;
    if game_daily_frequency.as_rule() != Rule::uint {
        return Err(ParseError::Placeholder);
    }
    let game_daily_frequency = game_daily_frequency
        .as_str()
        .replace("_", "")
        .parse()
        .ok()
        .ok_or(ParseError::Placeholder)?;

    let health_cost = inner.next().ok_or(ParseError::Placeholder)?;
    if health_cost.as_rule() != Rule::uint {
        return Err(ParseError::Placeholder);
    }
    let health_cost = health_cost
        .as_str()
        .replace("_", "")
        .parse()
        .ok()
        .ok_or(ParseError::Placeholder)?;

    let mut input: HashMap<String, u64> = HashMap::new();
    let blist = inner.next().ok_or(ParseError::Placeholder)?;
    if blist.as_rule() != Rule::blist {
        return Err(ParseError::Placeholder);
    }
    for item in blist.into_inner() {
        if item.as_rule() != Rule::upair {
            return Err(ParseError::Placeholder);
        }
        let mut tmp = item.into_inner();

        let key = tmp
            .next()
            .ok_or(ParseError::Placeholder)?
            .as_str()
            .to_owned();
        let val = tmp
            .next()
            .ok_or(ParseError::Placeholder)?
            .as_str()
            .replace("_", "")
            .parse()
            .ok()
            .ok_or(ParseError::Placeholder)?;

        input.insert(key, val);
    }

    let output_quantity = inner.next().ok_or(ParseError::Placeholder)?;
    if output_quantity.as_rule() != Rule::uint {
        return Err(ParseError::Placeholder);
    }
    let output_quantity = output_quantity
        .as_str()
        .replace("_", "")
        .parse::<u64>()
        .ok()
        .ok_or(ParseError::Placeholder)?;

    let output_name = inner.next().ok_or(ParseError::Placeholder)?;
    if output_name.as_rule() != Rule::ident {
        return Err(ParseError::Placeholder);
    }
    let output_name = output_name.as_str().to_owned();

    match pairs.as_rule() {
        Rule::item_config => {
            let item = Item { name: output_name };
            let output = (output_quantity, item);

            Ok(FactoryConfig::ItemConfig {
                name,
                game_daily_frequency,
                health_cost,
                input,
                output,
            })
        }
        Rule::product_config => {
            let mut properties: HashMap<String, Expr> = HashMap::new();
            let clist = inner.next().ok_or(ParseError::Placeholder)?;
            if clist.as_rule() != Rule::clist {
                return Err(ParseError::Placeholder);
            }
            for item in clist.into_inner() {
                if item.as_rule() != Rule::xpair {
                    return Err(ParseError::Placeholder);
                }
                let mut tmp = item.into_inner();

                let ident = tmp
                    .next()
                    .ok_or(ParseError::Placeholder)?
                    .as_str()
                    .to_owned();

                let expr = parse_expr(tmp.next().ok_or(ParseError::Placeholder)?.into_inner());

                properties.insert(ident, expr);
            }
            let product = Product {
                name: output_name,
                properties,
            };
            let output = (output_quantity, product);

            Ok(FactoryConfig::ProductConfig {
                name,
                game_daily_frequency,
                health_cost,
                input,
                output,
            })
        }

        _ => Err(ParseError::Placeholder),
    }
}
