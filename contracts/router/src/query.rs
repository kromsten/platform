use cosmwasm_std::{StdResult, Deps};
use secret_toolkit::utils::types::Token;

use crate::{state::{TokenStrategy, STRATEGY_ROUTER, Transformation, TRANSFORMATIONS}, utils::{unwrap_token, unwrap_fee}};



pub fn token_strategies(deps: Deps, token: Token) -> StdResult<Vec<TokenStrategy>> {
    let name = unwrap_token(&token).0;
    let suffix = name.as_bytes();

    let strategies : Vec<TokenStrategy> = STRATEGY_ROUTER
        .add_suffix(suffix)
        .paging(deps.storage, 0, 10)?;

    Ok(strategies)
}



pub fn transformations(deps: Deps, token_in: Token, token_out: Token) -> StdResult<Vec<Transformation>> {
    let name_in = unwrap_token(&token_in).0;
    let name_out = unwrap_token(&token_out).0;

    let mut transformations : Vec<Transformation> = TRANSFORMATIONS
        .add_suffix(name_in.as_bytes())
        .add_suffix(name_out.as_bytes())
        .iter(deps.storage)?
        .map(|it| it.unwrap() )
        .collect();

    if transformations.len() > 1 {
        transformations.sort_by(
            |a, b|  unwrap_fee(&a.fee).cmp(&unwrap_fee(&b.fee))
        );
    }

    Ok(transformations)
}



pub fn calculate_strategies(deps: Deps, token: Token) -> StdResult<Vec<TokenStrategy>> {
    let normal_strategies : Vec<TokenStrategy> = token_strategies(deps, token.clone())?;
    let strategies = normal_strategies;
    Ok(strategies)
}