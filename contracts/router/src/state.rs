use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use secret_toolkit::{
    storage::{
        Item, AppendStore
    }, 
    utils::types::{Token, Contract}, 
    serialization::Json
};
use strategy::{InvestmentAction};

use crate::msg::TransformationFee;



pub static CONFIG_KEY: &[u8] = b"config";




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TokenStrategy {
    pub contract: Contract,
    pub inputs: Vec<Token>,
    pub outputs: Vec<Token>,
    pub apr: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Transformation {
    pub actions: InvestmentAction,
    pub fee: TransformationFee,
}


pub static ADMIN : Item<Addr> = Item::new(b"admin");
pub static STRATEGY_ROUTER: AppendStore<TokenStrategy, Json> = AppendStore::new(b"strategy_router");
pub static TRANSFORMATIONS: AppendStore<Transformation, Json> = AppendStore::new(b"transformations");

