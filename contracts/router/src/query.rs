use cosmwasm_std::{StdResult, Deps};
use secret_toolkit::utils::types::Token;

use crate::state::{Strategy, STRATEGY_ROUTER};


pub fn all_strategies(deps: Deps) -> StdResult<Vec<Strategy>> {
    let strategies : Vec<Strategy> = STRATEGY_ROUTER
        .iter(deps.storage)?
        .map(|item| item.unwrap().1)
        .collect();

    Ok(strategies)
}


pub fn strategies(_deps: Deps, _token: Token) -> StdResult<Vec<Strategy>> {
    
    Ok(vec![])
}