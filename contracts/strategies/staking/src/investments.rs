use crate::{
    contract::MAIN_CHAIN_ID, 
    msg::{InvestmentMsg}, 
    attributes::{validator_attribute, delegator_attribute, amount_attribute}
};


pub fn invest_msg() -> InvestmentMsg {
    InvestmentMsg {
        chain_id: MAIN_CHAIN_ID.into(),
        exposes_investor: true, 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        attributes: vec![
            validator_attribute(None, true),
            delegator_attribute(),
            amount_attribute(),
        ] 
    }
}


pub fn withdraw_msg(
    validator : Option<&String>, 
) -> InvestmentMsg {
    InvestmentMsg {
        chain_id: MAIN_CHAIN_ID.into(),
        exposes_investor: true, 
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(), 
        attributes: vec![
            validator_attribute(validator, true),
            delegator_attribute(),
        ] 
    }
}



pub fn claim_msg(
    validator : Option<&String>,
) -> InvestmentMsg {
    InvestmentMsg {
        chain_id: MAIN_CHAIN_ID.into(),
        exposes_investor: true, 
        type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward".to_string(), 
        attributes: vec![
            validator_attribute(validator, true),
            delegator_attribute(),
        ] 
    }
}
