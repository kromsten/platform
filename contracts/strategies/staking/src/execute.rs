use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, CosmosMsg, to_binary, coin, Uint128, StdError};

use crate::{msg::{DelegateMsg, ClaimMsg}, state::config_read};


pub fn try_invest(
    deps: DepsMut, 
    info: MessageInfo,
    amount: Uint128,
    validator_address: Option<String>,
    delegator_address: Option<String>,
) -> StdResult<Response> {

    let config = config_read(deps.storage).load()?;
    let validator = validator_address.unwrap_or(config.default_validator.to_string());
    let delegator = delegator_address.unwrap_or(info.sender.to_string());
    
    let msg = CosmosMsg::Stargate { 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        value: to_binary(&DelegateMsg {
            delegator_address: delegator,
            validator_address: validator,
            amount: coin(amount.u128(), "uscrt"),
        })?,
    };

    Ok(Response::default().add_message(msg))
}


pub fn try_withdraw(
    deps: DepsMut, 
    info: MessageInfo,
    amount: Uint128,
    validator_address: Option<String>,
    delegator_address: Option<String>,
) -> StdResult<Response> {

    let delegator = delegator_address.unwrap_or(info.sender.to_string());

    let mut msgs : Vec<CosmosMsg> = Vec::new();

    if validator_address.is_some() {
        msgs.push(CosmosMsg::Stargate { 
            type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(), 
            value: to_binary(&DelegateMsg {
                delegator_address: delegator,
                validator_address: validator_address.unwrap(),
                amount: coin(amount.u128(), "uscrt"),
            })?,
        })
    } else {
        let delegations = deps.querier.query_all_delegations(delegator.clone()).unwrap();

        let total_bonded: Uint128 = delegations
            .iter()
            .map(|delegation| delegation.amount.amount)
            .sum();

        if total_bonded < amount {
            return Err(StdError::generic_err("Not enough bonded tokens"));
        }

        let mut left = amount;

        for delegation in delegations {

            let to_deduct = if left > delegation.amount.amount {
                delegation.amount.amount
            } else {
                left
            };

            msgs.push(CosmosMsg::Stargate { 
                type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(), 
                value: to_binary(&DelegateMsg {
                    delegator_address: delegator.clone(),
                    validator_address: delegation.validator,
                    amount: coin(to_deduct.u128(), "uscrt"),
                })?,
            });

            left -= to_deduct;

            if left == Uint128::zero() {
                break;
            }
        }

    }

    Ok(Response::default().add_messages(msgs))
}



pub fn try_claim(
    deps: DepsMut,
    info: MessageInfo,
    validator_address: Option<String>,
    delegator_address: Option<String>,
) -> StdResult<Response> {

    let delegator = delegator_address.unwrap_or(info.sender.to_string());

    let mut msgs : Vec<CosmosMsg> = Vec::new();

    if validator_address.is_some() {
        msgs.push(CosmosMsg::Stargate { 
            type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward".to_string(), 
            value: to_binary(&ClaimMsg {
                delegator_address: delegator,
                validator_address: validator_address.unwrap()
            })?,
        })
    } else {

        let delegations = deps.querier.query_all_delegations(delegator).unwrap();

        for delegation in delegations {

            msgs.push(CosmosMsg::Stargate { 
                type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward".to_string(), 
                value: to_binary(&ClaimMsg {
                    delegator_address: delegation.delegator.into_string(),
                    validator_address: delegation.validator
                })?,
            })

        }
    }

    Ok(Response::default().add_messages(msgs))
}