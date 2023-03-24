use crate::expr::*;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub properties: HashMap<String, Expr>,
}
#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
}
#[derive(Debug, Clone)]
pub enum FactoryConfig {
    ProductConfig {
        name: String,
        game_daily_frequency: u64,
        health_cost: u64,
        input: HashMap<String, u64>,
        output: (u64, Product),
    },
    ItemConfig {
        name: String,
        game_daily_frequency: u64,
        health_cost: u64,
        input: HashMap<String, u64>,
        output: (u64, Item),
    },
}
impl FactoryConfig {
    pub fn blank_product() -> Self {
        Self::ProductConfig {
            name: "PLACEHOLDER".into(),
            game_daily_frequency: 0,
            health_cost: 0,
            input: HashMap::new(),
            output: (
                0,
                Product {
                    name: "PLACEHOLDER".into(),
                    properties: HashMap::new(),
                },
            ),
        }
    }

    pub fn blank_item() -> Self {
        Self::ItemConfig {
            name: "PLACEHOLDER".into(),
            game_daily_frequency: 0,
            health_cost: 0,
            input: HashMap::new(),
            output: (
                0,
                Item {
                    name: "PLACEHOLDER".into(),
                },
            ),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: old,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: _,
            } => {
                *old = name.to_owned();
                self
            }
            ProductConfig {
                name: old,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: _,
            } => {
                *old = name.to_owned();
                self
            }
        }
    }

    pub fn game_daily_frequency(&mut self, freq: u64) -> &mut Self {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: _,
                game_daily_frequency: old,
                health_cost: _,
                input: _,
                output: _,
            } => {
                *old = freq;
                self
            }
            ProductConfig {
                name: _,
                game_daily_frequency: old,
                health_cost: _,
                input: _,
                output: _,
            } => {
                *old = freq;
                self
            }
        }
    }

    pub fn health_cost(&mut self, cost: u64) -> &mut Self {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: old,
                input: _,
                output: _,
            } => {
                *old = cost;
                self
            }
            ProductConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: old,
                input: _,
                output: _,
            } => {
                *old = cost;
                self
            }
        }
    }

    pub fn input(&mut self, input: HashMap<String, u64>) -> &mut Self {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: old,
                output: _,
            } => {
                *old = input;
                self
            }
            ProductConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: old,
                output: _,
            } => {
                *old = input;
                self
            }
        }
    }

    pub fn output_quantity(&mut self, input: u64) -> &mut Self {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: (old, _),
            } => {
                *old = input;
                self
            }
            ProductConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: (old, _),
            } => {
                *old = input;
                self
            }
        }
    }

    /// returns none if self is ItemConfig. It should be ProductConfig
    pub fn output_product(&mut self, output: Product) -> Option<&mut Self> {
        use FactoryConfig::*;
        match self {
            ProductConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: (_, old),
            } => {
                *old = output;
                Some(self)
            }
            _ => None,
        }
    }

    /// returns none if self is ProductConfig. It should be ItemConfig
    pub fn output_item(&mut self, output: Item) -> Option<&mut Self> {
        use FactoryConfig::*;
        match self {
            ItemConfig {
                name: _,
                game_daily_frequency: _,
                health_cost: _,
                input: _,
                output: (_, old),
            } => {
                *old = output;
                Some(self)
            }
            _ => None,
        }
    }
}
