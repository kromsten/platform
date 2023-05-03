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

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}


#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Invest { 
            amount, 
            validator_address,
            delegator_address, 
        } => try_invest(deps, info, amount, validator_address, delegator_address),

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
    }
}






pub fn test_paramas(
    _deps: Deps,
) -> StdResult<bool> {
    Ok(true)
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


#[cfg(test)]
mod tests {
    use crate::msg::{InvestParamsResult, Attribute};

    use super::*;
    use cosmwasm_std::{testing::*, Api, OwnedDeps, MemoryStorage, Addr};
    use cosmwasm_std::{from_binary, Coin, Uint128};

    fn attribute_checker(
        deps : &OwnedDeps<MemoryStorage, MockApi, MockQuerier>, 
        attr: Attribute
    )  {

        if attr.querier.is_some() {
            let q = attr.querier.unwrap();

            let res = deps.querier.handle_query(&q.request).unwrap();

            deps.api.debug(&format!("res: {:?}", res));

        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let init_msg = InstantiateMsg { admin: None, default_validator: Addr::unchecked("validator") };
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        assert_eq!(0, res.messages.len());
    }




    #[test]
    fn query_invest_params() {
        let mut deps = mock_dependencies();

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { admin: None, default_validator: Addr::unchecked("validator")};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();



        let qmsg : QueryMsg = QueryMsg::InvestParams {  };

        let result = query(deps.as_ref(), mock_env(), qmsg);

        assert_eq!(result.is_ok(), true);

        let response : InvestParamsResult = from_binary(&result.unwrap()).unwrap();

        deps.api.debug(&format!("invest params res: {:?}", response));

        response.attributes
            .iter()
            .for_each(|attr| attribute_checker(&deps, attr.clone()));


        assert_eq!(1, 2)


    }

}