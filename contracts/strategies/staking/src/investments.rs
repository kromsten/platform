use crate::{
    contract::MAIN_CHAIN_ID, 
    msg::{InvestmentAction, ActionClass, ActionRequrement}, 
    attributes::{validator_attribute, delegator_attribute, amount_attribute}
};


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
        independent_action_requirements: Some(vec![ActionRequrement::Authz {}])
    }
}


pub fn invest_msg() -> InvestmentAction {
    InvestmentAction {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        attributes: vec![
            validator_attribute(None, true),
            delegator_attribute(),
            amount_attribute(),
        ],
        ..default_msg()
    }
}


pub fn withdraw_msg(
    validator : Option<&String>, 
) -> InvestmentAction {
    InvestmentAction {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        attributes: vec![
            validator_attribute(validator, true),
            delegator_attribute(),
        ],
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
        class: ActionClass::Claiming {},
        ..default_msg()
    }
}
