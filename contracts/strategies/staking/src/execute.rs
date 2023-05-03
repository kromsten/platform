use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, CosmosMsg, to_binary, coin, Uint128, StdError, Addr};

use crate::{msg::{DelegateMsg, ClaimMsg, AutoStakeMsg}, state::{config_read, config}};


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



pub fn try_auto_reinvest(
    deps: DepsMut,
    info: MessageInfo,
    validator_address: Option<String>,
    delegator_address: Option<String>,
    enabled: bool,
) -> StdResult<Response> {

    let delegator = delegator_address.unwrap_or(info.sender.to_string());

    let mut msgs : Vec<CosmosMsg> = Vec::new();

    if validator_address.is_some() {
        msgs.push(CosmosMsg::Stargate { 
            type_url: "/cosmos.distribution.v1beta1.MsgSetAutoRestake".to_string(), 
            value: to_binary(&AutoStakeMsg {
                delegator_address: delegator,
                validator_address: validator_address.unwrap(),
                enabled
            })?,
        })
    } else {
            
        let delegations = deps.querier.query_all_delegations(delegator).unwrap();

        for delegation in delegations {

            msgs.push(CosmosMsg::Stargate { 
                type_url: "/cosmos.distribution.v1beta1.MsgSetAutoRestake".to_string(), 
                value: to_binary(&AutoStakeMsg {
                    delegator_address: delegation.delegator.into_string(),
                    validator_address: delegation.validator,
                    enabled,
                })?,
            })

        }
    }

    Ok(Response::default().add_messages(msgs))
}


pub fn try_change_config(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Option<Addr>,
    new_default_validator: Option<Addr>,
) -> StdResult<Response> {

    let mut state = config(deps.storage).load()?;
    
    if state.admin != deps.api.addr_canonicalize(&info.sender.to_string())? {
        return Err(StdError::GenericErr { msg: "Unauthorized".to_string() });
    }

    let mut attributes : Vec<(&str, String)> = Vec::new();

    if new_admin.is_some() {
        state.admin = deps.api.addr_canonicalize(&new_admin.clone().unwrap().to_string())?;
        attributes.push(("action", "change_admin".to_string()));
        attributes.push(("new_admin", new_admin.unwrap().to_string()));
    }

    if new_default_validator.is_some() {
        state.default_validator = deps.api.addr_canonicalize(&new_default_validator.clone().unwrap().to_string())?;
        attributes.push(("action", "change_default_validator".to_string()));
        attributes.push(("new_default_validator", new_default_validator.unwrap().to_string()));
    }

    if attributes.len() > 0 {
        config(deps.storage).save(&state)?;
    }


    Ok(Response::default()
        .add_attributes(attributes))
}