use cosmwasm_std::{DepsMut, Addr, StdResult, Response};
use crate::state::{STRATEGY_ROUTER, Strategy};


pub fn add_route(
    deps: DepsMut,
    address: Addr,
) -> StdResult<Response> {

    STRATEGY_ROUTER.insert(deps.storage, &address, &Strategy {}).unwrap();
    Ok(Response::default())
}