use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::execute::add_route;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::all_strategies;
use crate::state::{config, State};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    
    let state = State {
        admin: msg.admin.unwrap_or(info.sender),
    };

    deps.api.debug(format!("Contract was initialized by {}", state.admin).as_str());
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddRoute { address } => add_route(deps, address)
    }
}



#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllStrategies {} => to_binary(&all_strategies(deps)?)
    }
}

