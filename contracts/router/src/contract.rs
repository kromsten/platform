use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::execute::add_strategy;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::token_strategies;
use crate::state::{ADMIN};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let admin = msg.admin.unwrap_or(info.sender);
    ADMIN.save(deps.storage, &admin)?;
    deps.api.debug(format!("Contract was initialized by {}", admin).as_str());
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddStrategy { contract } => add_strategy(deps, info.sender, contract)
    }
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenStrategies { token } => to_binary(&token_strategies(deps, token)?)
    }
}

