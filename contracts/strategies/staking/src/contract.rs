use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, CosmosMsg, QueryRequest, Empty, StakingMsg, coin, DelegationResponse, Addr,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, DelegateMsg, Attribute, ValueQuerier, AttributeValue, InvestParamsResult};
use crate::state::{config, State};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: info.sender.clone(),
    };

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Invest { amount, validator } => try_invest(deps, info, validator, amount),
    }
}

pub fn try_increment(deps: DepsMut, _env: Env) -> StdResult<Response> {
    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, _count: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    config(deps.storage).update(|state| {
        if sender_address != state.owner {
            return Err(StdError::generic_err("Only the owner can reset count"));
        }
        Ok(state)
    })?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}


pub fn try_invest(
    _deps: DepsMut, 
    info: MessageInfo,
    validator: String,
    amount: u128,
) -> StdResult<Response> {
    
    
    let msg = CosmosMsg::Stargate { 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        value: to_binary(&DelegateMsg {
            delegator_address: info.sender.to_string(),
            validator_address: validator.clone(),
            amount: coin(amount, "uscrt"),
        })?,
    };

    Ok(Response::default().add_message(msg))
}


pub fn try_withdraw(
    _deps: DepsMut, 
    info: MessageInfo,
    validator: String,
    amount: u128,
) -> StdResult<Response> {
    
    
    let msg = CosmosMsg::Stargate { 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        value: to_binary(&DelegateMsg {
            delegator_address: info.sender.to_string(),
            validator_address: validator.clone(),
            amount: coin(amount, "uscrt"),
        })?,
    };
    
    Ok(Response::default().add_message(msg))
}





#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TestQuery { request } => 
            to_binary(&test_query(deps, request)?),
        QueryMsg::InvestParams { address } => 
            to_binary(&invest_params(deps, address)?),
    }
}


pub fn invest_params(
    _deps: Deps,
    _address: Addr
) -> StdResult<InvestParamsResult> {

    let attributes: Vec<Attribute> = vec![
        Attribute {
            key: String::from("validator_address"),
            value: Some(AttributeValue::Custom { 
                value_type: "String".to_string(), 
                value: to_binary(&String::from("secretvaloper1tzjf8wsy3keretlukuvr0hzwqsr76rjd9h9pdy"))?
            }),
            querier: Some(ValueQuerier {
                path: String::from("/cosmos.staking.v1beta1.Query/Validators"),
                msg: QueryRequest::Staking({
                    cosmwasm_std::StakingQuery::AllValidators {}
                }),
                group: 0, 
                jq_parser: String::from(".validators[].operator_address")
            })
        },

        Attribute {
            key: String::from("delegator_address"),
            value: Some(AttributeValue::Sender {}),
            querier: None
        },


        Attribute {
            key: String::from("amount"),
            value: Some(AttributeValue::TokenBalance { 
                token: secret_toolkit::utils::types::Token::Native("uscrt".to_string()),
                amount: 10
            }),
            querier: None
        },
    ];

    Ok(InvestParamsResult { 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        attributes 
    })
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
    use crate::msg::InvestParamsResult;

    use super::*;
    use cosmwasm_std::{testing::*, Api, OwnedDeps, MemoryStorage};
    use cosmwasm_std::{from_binary, Coin, Uint128};
    use chumsky::{Parser};
    //use jaq_parse::{parse};

    fn attribute_checker(
        deps : &OwnedDeps<MemoryStorage, MockApi, MockQuerier>, 
        attr: Attribute
    )  {

        if attr.querier.is_some() {
            let q = attr.querier.unwrap();

            let res = deps.querier.handle_query(&q.msg).unwrap();

            deps.api.debug(&format!("res: {:?}", res));


            /* let parsed = parse(
                "{ 'test': { 'a': 'b' } }",
                
            );  */
           

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
        let init_msg = InstantiateMsg { };
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
        let init_msg = InstantiateMsg {};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();



        let qmsg : QueryMsg = QueryMsg::InvestParams { address: info.sender };

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