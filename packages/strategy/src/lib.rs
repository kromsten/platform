mod msg;
mod investments;
mod attributes;


pub use crate::msg::{
    InvestmentAction, InvestParamsResult, 
    ActionClass, ActionRequrement, IssuedToken, Reward, 
    Attribute, AttributeValue, QueryMsg,
    ValueQuerier, QueryBuilder, RequestBuilder,
    RewardResponse, RewardsQueryResponse,
    MessageBuilder
};

pub use attributes::{
    validator_attribute, delegator_attribute, amount_attribute, coin_amount_attribute
};

pub use investments::default_msg;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // test me
    }
}