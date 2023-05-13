use cosmwasm_std::{Env};

use crate::{
    contract::MAIN_CHAIN_ID, 
    attributes::{validator_attribute, delegator_attribute, coin_amount_attribute}
};


use strategy::{InvestmentAction, ActionClass, ActionRequrement};



fn default_msg() -> InvestmentAction {
    InvestmentAction { 
        chain_id: MAIN_CHAIN_ID.into(), 
        type_url: String::new(), 
        attributes: Vec::new(), 
        exposes_investor: true, 
        issued_tokens: None, 
        optional: false,
        description: None,
        class: ActionClass::Staking {},
        action_requirements: None,
        independent_action_requirements: Some(vec![ActionRequrement::Authz {}]),
        unbonding: None,
    }
}


pub fn invest_msg() -> InvestmentAction {
    InvestmentAction {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        attributes: vec![
            validator_attribute(None, true),
            delegator_attribute(),
            coin_amount_attribute(),
        ],
        description: Some("Native message for delegation SCRT tokens".to_string()),
        ..default_msg()
    }
}


pub fn withdraw_msg(
    env: Env,
    validator : Option<&String>, 
) -> InvestmentAction {
    InvestmentAction {
        type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(), 
        attributes: vec![
            validator_attribute(validator, true),
            delegator_attribute(),
            coin_amount_attribute()
        ],
        description: Some("Native message for undelegation SCRT tokens that start 21 days unbonding process".to_string()),
        unbonding: Some(env.block.time.plus_seconds(1814400)),
        ..default_msg()
    }
}


pub fn claim_msg(
    validator : Option<&String>,
) -> InvestmentAction {
    InvestmentAction {
        type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward".to_string(), 
        attributes: vec![
            validator_attribute(validator, true),
            delegator_attribute(),
        ],
        description: Some("Claim distribution rewards from bounded SCRTs".to_string()),
        class: ActionClass::Claiming {},
        ..default_msg()
    }
}
