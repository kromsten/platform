use cosmwasm_std::{StdResult, Deps};
use secret_toolkit::utils::types::Token;

use crate::{state::{TokenStrategy, STRATEGY_ROUTER}, utils::unwrap_token};



pub fn token_strategies(deps: Deps, token: Token) -> StdResult<Vec<TokenStrategy>> {
    let name = unwrap_token(&token).0;
    let suffix = name.as_bytes();

    let strategies : Vec<TokenStrategy> = STRATEGY_ROUTER
        .add_suffix(suffix)
        .iter(deps.storage)?
        .map(|item| item.unwrap().1)
        .collect(); 

    Ok(strategies)
}