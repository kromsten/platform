use cosmwasm_std::{Addr, Binary, QueryRequest, Empty, Coin, Uint128};
use schemars::JsonSchema;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};

/* 
pub const banka : QueryRequest<Empty> = QueryRequest::Bank( {
    cosmwasm_std::BankQuery::Balance { address: (), denom: () }
}) */

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
    pub default_validator: Addr,
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    
    Invest { 
        amount: Uint128, 
        validator_address: Option<String>, 
        delegator_address: Option<String>
    },
    
    Withdraw {
        amount: Uint128, 
        validator_address: Option<String>, 
        delegator_address: Option<String>
    },
    
    Claim {
        validator_address: Option<String>, 
        delegator_address: Option<String>
    },

    ActivateReinvest {
        validator_address: Option<String>, 
        delegator_address: Option<String>
    },

    DeactivateReinvest {
        validator_address: Option<String>, 
        delegator_address: Option<String>
    },

    ChangeConfig {
        admin: Option<Addr>,
        default_validator: Option<Addr>,
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TestQuery { request: QueryRequest<Empty> },

    InvestParams {},
    WithdrawParams {},
    ClaimParams {},

    RewardsQuery {},
    
    AllRewards { },
    Rewards { token: Token },

    InvestMsgs {},
    WithdrawMsgs { address: Option<Addr> },
    ClaimMsgs { address: Option<Addr> },
    
    InvestTokens {},
    RewardTokens {},

    WithPermit {
        query: QueryRequest<Empty>,
    }
}





#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ValueQuerier {
    pub path: String,
    pub request: QueryRequest<Empty>,
    pub group: u8,
    pub jq_parser: String,

}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeValue {
    Investor {},
    Contract {},
    Amount {},
    Custom {
        value_type: String,
        value: Binary
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Attribute {
    pub key: String,
    pub optional: bool,
    pub value: Option<AttributeValue>,
    pub querier: Option<ValueQuerier>
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct QueryBuilder {
    pub key: String,
    pub value: Option<AttributeValue>,
    pub querier: Option<ValueQuerier>
}




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct DelegateMsg {
    pub delegator_address: String,
    pub validator_address: String,
    pub amount: Coin
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ClaimMsg {
    pub delegator_address: String,
    pub validator_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct AutoStakeMsg {
    pub delegator_address: String,
    pub validator_address: String,
    pub enabled: bool
}




#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentMsg {
    pub chain_id: String,
    pub type_url: String,
    pub attributes: Vec<Attribute>,
    pub exposes_investor: bool
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RequestBuilder {
    pub name: String,
    pub attributes: Vec<Attribute>
}

pub type InvestParamsResult = RequestBuilder;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Reward {
    token: Token,
    amount: Uint128
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RewardResponse {
    pub rewards: Reward
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RewardsResponse {
    pub rewards: Vec<Reward>
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RewardsQueryResponse {
    pub path: String,
    pub request: Option<QueryRequest<Empty>>,
    pub request_builder: Option<RequestBuilder>
}
