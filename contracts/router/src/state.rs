use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use secret_toolkit::storage::{Keymap, KeymapBuilder};



pub static CONFIG_KEY: &[u8] = b"config";




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Strategy {}



pub static STRATEGY_ROUTER: Keymap<Addr, Strategy> = KeymapBuilder::new(b"strategy_router")
    .with_page_size(10)
    .build();



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr
}



pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
