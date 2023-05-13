use std::str::FromStr;

use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, QueryRequest, Empty, DelegationResponse,
    entry_point, to_binary, Decimal
};

use crate::execute::{try_invest, try_withdraw, try_claim, try_change_config, try_auto_reinvest};
use crate::msg::{
    ExecuteMsg, 
    InstantiateMsg, 
};

use strategy::QueryMsg;

use crate::query::{
    invest_params, 
    withdraw_params, 
    claim_params, 
    invest_messages, 
    withdraw_messages, 
    claim_messages, 
    rewards_query,
    tokens, not_implemented, apr, strategy_info, strategy_full_info, 
};

use crate::state::{config, Config};

pub const MAIN_CHAIN_ID : &str = "secret-4";


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    let state = Config {
        admin: deps.api.addr_canonicalize(&msg.admin.unwrap_or(info.sender.clone()).to_string())?,
        default_validator: msg.default_validator,
        can_query_rewards: false,
        native_reinvest: false, // not imlemented yet
        private_queries: false,
        description: String::from("Native Secret Network Staking Strategy"),
        apr: Decimal::from_str("20.5")?
    };

    deps.api.debug(format!("Contract was initialized by {}", info.sender).as_str());
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}


#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Invest { 
            amount, 
            validator_address,
            delegator_address, 
        } => try_invest(deps, env, info, amount, validator_address, delegator_address),

        ExecuteMsg::Withdraw { 
            amount, 
            validator_address, 
            delegator_address 
        } => try_withdraw(deps, env, info, amount, validator_address, delegator_address),

        ExecuteMsg::Claim { 
            validator_address, 
            delegator_address 
        } => try_claim(deps, env, info, validator_address, delegator_address),

        ExecuteMsg::ActivateReinvest { 
            validator_address, 
            delegator_address 
        } => try_auto_reinvest(deps, env, info, validator_address, delegator_address, true),

        ExecuteMsg::DeactivateReinvest { 
            validator_address, 
            delegator_address 
        } => try_auto_reinvest(deps, env, info, validator_address, delegator_address, false),

        ExecuteMsg::ChangeConfig { 
            admin, 
            default_validator 
        } => try_change_config(deps, info, admin, default_validator),
    }
}



#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::InvestParams {} =>  to_binary(&invest_params()?),
        QueryMsg::WithdrawParams {} =>  to_binary(&withdraw_params()?),
        QueryMsg::ClaimParams {} =>  to_binary(&claim_params()?),

        QueryMsg::AllRewards {} => to_binary(&not_implemented()?),
        QueryMsg::Rewards { token: _ } => to_binary(&not_implemented()?),
        QueryMsg::Apr {} => to_binary(&apr(deps)?),
        
        QueryMsg::AprQuery {} => to_binary(&not_implemented()?),
        QueryMsg::RewardsQuery {} => to_binary(&rewards_query(deps)?),
        
        QueryMsg::InvestMsgs {} => to_binary(&invest_messages()?),
        QueryMsg::WithdrawMsgs {address } => to_binary(&withdraw_messages(deps, env, address)?),
        QueryMsg::ClaimMsgs { address } => to_binary(&claim_messages(deps, address)?),

        QueryMsg::InvestTokens {} => to_binary(&tokens()?),
        QueryMsg::RewardTokens {} => to_binary(&tokens()?),

        QueryMsg::WithPermit { query: _ } => to_binary(&not_implemented()?),

        QueryMsg::StrategyInfo {} => to_binary(&strategy_info(deps)?),
        QueryMsg::StrategyFullInfo { address } => to_binary(&strategy_full_info(deps, env, address)?),

        QueryMsg::TestQuery { 
            path,
            query 
        } => to_binary(&test_paramas(deps, path, query)?),
    }
}






pub fn test_paramas(
    deps: Deps,
    path: String,
    data: Binary,
) -> StdResult<Binary> {

    let query : QueryRequest<Empty> = QueryRequest::Stargate {
        path: path,
        data,
    };


    let res = deps.querier.query(&query);

    deps.api.debug(&format!("query res: {:?}", res));

    let unwrapped = res.unwrap();

    deps.api.debug(&format!("query unwrapped: {:?}", unwrapped));

    Ok(unwrapped)
}



pub fn test_query(
    deps: Deps,
    request: QueryRequest<Empty>,
) -> StdResult<bool> {

    let querier = deps.querier;

    let res: DelegationResponse = querier.query(&request)?;

    deps.api.debug(&format!("query res: {:?}", res));

    Ok(true)
}


