use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::execute::add_route;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::all_strategies;
use crate::state::{ADMIN};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    let admin = msg.admin.unwrap_or(info.sender);
    ADMIN.save(deps.storage, &admin)?;
    deps.api.debug(format!("Contract was initialized by {}", admin).as_str());
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddRoute { contract } => add_route(deps, info.sender, contract)
    }
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllStrategies {} => to_binary(&all_strategies(deps)?)
    }
}

