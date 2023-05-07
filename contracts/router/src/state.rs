use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use secret_toolkit::{
    storage::{
        Keymap, KeymapBuilder, Item
    }, 
    utils::types::Token, 
    serialization::Json
};



pub static CONFIG_KEY: &[u8] = b"config";


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TokenStrategy {
    pub inputs: Vec<Token>,
    pub outputs: Vec<Token>,

}


pub const ADMIN : Item<Addr> = Item::new(b"admin");


pub static STRATEGY_ROUTER: Keymap<String, TokenStrategy, Json> = KeymapBuilder::new(b"strategy_router")
    .with_page_size(10)
    .build();


