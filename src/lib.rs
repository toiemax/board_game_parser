#![doc = "Documentation"]

use pest::iterators::Pair;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    name: String,
    author: String,
    age: u8,
    min_time: u16,
    max_time: Option<u16>,
    min_players: u8,
    max_players: Option<u8>,
    price: f64,
}

#[derive(Error, Debug)]
pub enum GameParseError {
    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid price format: {0}")]
    InvalidPriceFormat(String),
}

impl Game {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut name = String::new();
        let mut author = String::new();
        let mut age = 0;
        let mut min_time = 0;
        let mut max_time = None;
        let mut min_players = 0;
        let mut max_players = None;
        let mut price = 0.0;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::name => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();  
                    name = inner.next().map_or(String::new(), |p| p.as_str().trim().to_string());
                }
                Rule::author => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();  
                    author = inner.next().map_or(String::new(), |p| p.as_str().trim().to_string());
                }
                Rule::age => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();  
                    age = inner.next().map_or(0, |p| p.as_str().trim().parse::<u8>().unwrap_or(0));
                }
                Rule::time => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();  
                    min_time = inner.next().map_or(0, |p| p.as_str().trim().parse::<u16>().unwrap_or(0));
                    max_time = inner.next().map_or(None, |p| p.as_str().trim().parse::<u16>().ok());
                }
                Rule::players => {
                    let mut inner = inner_pair.into_inner();
                    inner.next(); 
                    min_players = inner.next().map_or(0, |p| p.as_str().trim().parse::<u8>().unwrap_or(0));
                    max_players = inner.next().map_or(None, |p| p.as_str().trim().parse::<u8>().ok());
                }
                Rule::price => {
                    let mut inner = inner_pair.into_inner();
                    inner.next();  
                    price = inner.next().map_or(0.0, |p| p.as_str().trim().parse::<f64>().unwrap_or(0.0));
                }
                _ => {}
            }
        }

        Game {
            name,
            author,
            age,
            min_time,
            max_time,
            min_players,
            max_players,
            price,
        }
    }
}
