use cosmwasm_std::{StdResult, Deps, Addr, Env};
use secret_toolkit::utils::types::Token;

use crate::{
    msg::{InvestParamsResult, InvestmentAction, RewardResponse, RewardsQueryResponse, RequestBuilder}, 
    attributes::{invest_attributes, claim_attributes, delegator_attribute, validator_attribute}, 
    investments::{invest_msg, withdraw_msg, claim_msg}
};


pub fn tokens() -> StdResult<Vec<Token>> {
    Ok(vec![Token::Native("uscrt".to_string())])
}


pub fn invest_params() -> StdResult<InvestParamsResult> {
    Ok(InvestParamsResult { 
        name: String::from("invest"), 
        attributes: invest_attributes(None)
    })
}


pub fn withdraw_params() -> StdResult<InvestParamsResult> {
    Ok(InvestParamsResult {
        name: String::from("withdraw"), 
        attributes: invest_attributes(None)
    })
}


pub fn claim_params() -> StdResult<InvestParamsResult> {
    Ok(InvestParamsResult {
        name: String::from("claim"),
        attributes: claim_attributes(None)
    })
}

pub fn not_implemented() -> StdResult<RewardResponse> {
    Err(cosmwasm_std::StdError::GenericErr { msg: "Not implemented".to_string() })
}



pub fn rewards_query(_deps: Deps) -> StdResult<RewardsQueryResponse> {
    Ok(RewardsQueryResponse { 
        path: String::from("/cosmos.distribution.v1beta1.Query/DelegationTotalRewards"), 
        request: None, 
        request_builder: Some(RequestBuilder {
            name: String::from(""),
            attributes: vec![
                validator_attribute(None, true),
                delegator_attribute(),
            ]
        }) 
    })
}



pub fn invest_messages() -> StdResult<Vec<InvestmentAction>> {
    Ok(vec![invest_msg()])
}


pub fn withdraw_messages(deps: Deps, env: Env, address : Option<Addr>) -> StdResult<Vec<InvestmentAction>> {
    let msgs: Vec<InvestmentAction> = if address.is_none() {
        vec![withdraw_msg(env.clone(), None)]
    } else {
        let delegations = deps.querier.query_all_delegations(address.unwrap().to_string()).unwrap();

        if delegations.len() == 0 {
            vec![]
        }  else {
            delegations.iter()
            .map(|delegation | withdraw_msg(env.clone(), Some(&delegation.validator)))
            .collect()
        }
    };
    Ok(msgs)
}


pub fn claim_messages(deps: Deps, address : Option<Addr>) -> StdResult<Vec<InvestmentAction>> {
    let msgs: Vec<InvestmentAction> = if address.is_none() {
        vec![claim_msg(None)]
    } else {
        let delegations = deps.querier.query_all_delegations(address.unwrap().to_string()).unwrap();

        if delegations.len() == 0 {
            vec![]
        }  else {
            delegations.iter()
            .map(|delegation | claim_msg(Some(&delegation.validator)))
            .collect()
        }
    };
    Ok(msgs)
}
