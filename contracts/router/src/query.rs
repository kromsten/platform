use cosmwasm_std::{StdResult, Deps};

use crate::state::{Strategy, STRATEGY_ROUTER};


pub fn all_strategies(deps: Deps) -> StdResult<Vec<Strategy>> {
    let strategies : Vec<Strategy> = STRATEGY_ROUTER
        .iter(deps.storage)?
        .map(|item| item.unwrap().1)
        .collect();

    Ok(strategies)
}