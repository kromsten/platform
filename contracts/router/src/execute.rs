use cosmwasm_std::{DepsMut, Addr, StdResult, StdError, Response};
use crate::state::{STRATEGY_ROUTER, Strategy, ADMIN};


pub fn add_route(
    deps: DepsMut,
    sender: Addr,
    address: Addr,
) -> StdResult<Response> {

    if ADMIN.load(deps.storage).unwrap() != sender {
        return Err(StdError::GenericErr { msg: "Only admin can add routes".to_string() });
    }

    STRATEGY_ROUTER.insert(deps.storage, &address, &Strategy {}).unwrap();
    Ok(Response::default())
}