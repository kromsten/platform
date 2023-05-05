use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, QueryRequest, Empty, DelegationResponse,
    entry_point, to_binary
};

use crate::execute::{try_invest, try_withdraw, try_claim, try_change_config, try_auto_reinvest};
use crate::msg::{
    ExecuteMsg, 
    InstantiateMsg, 
    QueryMsg
};

use crate::query::{
    invest_params, 
    withdraw_params, 
    claim_params, 
    invest_messages, 
    withdraw_messages, 
    claim_messages, 
    rewards_query,
    tokens, not_implemented, 
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
        native_reinvest: true,
        private_queries: false,
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
        } => try_withdraw(deps, info, amount, validator_address, delegator_address),

        ExecuteMsg::Claim { 
            validator_address, 
            delegator_address 
        } => try_claim(deps, info, validator_address, delegator_address),

        ExecuteMsg::ActivateReinvest { 
            validator_address, 
            delegator_address 
        } => try_auto_reinvest(deps, info, validator_address, delegator_address, true),

        ExecuteMsg::DeactivateReinvest { 
            validator_address, 
            delegator_address 
        } => try_auto_reinvest(deps, info, validator_address, delegator_address, false),

        ExecuteMsg::ChangeConfig { 
            admin, 
            default_validator 
        } => try_change_config(deps, info, admin, default_validator),
    }
}



#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::InvestParams {} =>  to_binary(&invest_params()?),
        QueryMsg::WithdrawParams {} =>  to_binary(&withdraw_params()?),
        QueryMsg::ClaimParams {} =>  to_binary(&claim_params()?),

        QueryMsg::AllRewards {} => to_binary(&not_implemented()?),
        QueryMsg::Rewards { token: _ } => to_binary(&not_implemented()?),
        QueryMsg::Apr {} => to_binary(&not_implemented()?),
        
        QueryMsg::AprQuery {  } => to_binary(&not_implemented()?),
        QueryMsg::RewardsQuery {} => to_binary(&rewards_query(deps)?),
        
        QueryMsg::InvestMsgs {} => to_binary(&invest_messages()?),
        QueryMsg::WithdrawMsgs {address } => to_binary(&withdraw_messages(deps, address)?),
        QueryMsg::ClaimMsgs { address } => to_binary(&claim_messages(deps, address)?),

        QueryMsg::InvestTokens {} => to_binary(&tokens()?),
        QueryMsg::RewardTokens {} => to_binary(&tokens()?),

        QueryMsg::WithPermit { query: _ } => to_binary(&not_implemented()?),

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

    let query = QueryRequest::Stargate {
        path: path,
        data,
    };
    
    deps.querier.query(&query)
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


