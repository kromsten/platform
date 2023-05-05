use cosmwasm_std::{to_binary, QueryRequest};

use crate::msg::{Attribute, AttributeValue, ValueQuerier};

pub const ALL_VALDATORS_GROUP : u8 = 0;

pub fn validator_attribute(
    validator : Option<&String>,
    optional : bool
) -> Attribute {
    Attribute {
        key: String::from("validator_address"),
        value: if validator.is_some() {
            Some(AttributeValue::Custom { 
                value_type: "String".to_string(), 
                value: to_binary(validator.unwrap()).unwrap()
            })
        }  else { None }
        ,
        querier: if validator.is_none() { 
            Some(ValueQuerier {
                path: String::from("/cosmos.staking.v1beta1.Query/Validators"),
                request: QueryRequest::Staking({
                    cosmwasm_std::StakingQuery::AllValidators {}
                }),
                group: ALL_VALDATORS_GROUP, 
                jq_parser: String::from(".validators | map(.operator_address)")
            })
        } else { None },
        optional
    }
    
}


pub fn delegator_attribute() -> Attribute {
    Attribute {
        key: String::from("delegator_address"),
        value: Some(AttributeValue::Investor {}),
        querier: None,
        optional: true
    }
}


pub fn amount_attribute() -> Attribute {
    Attribute {
        key: String::from("amount"),
        value: Some(AttributeValue::Amount {}),
        querier: None,
        optional: false
    }
}

pub fn coin_amount_attribute() -> Attribute {
    Attribute {
        key: String::from("amount"),
        value: Some(AttributeValue::CoinAmount {}),
        querier: None,
        optional: false
    }
}



pub fn invest_attributes(validator : Option<&String>) -> Vec<Attribute> {
    vec![
        validator_attribute(validator, true),
        delegator_attribute(),
        amount_attribute()
    ]
}

pub fn claim_attributes(validator : Option<&String>) -> Vec<Attribute> {
    vec![
        validator_attribute(validator, true),
        delegator_attribute(),
    ]
}




