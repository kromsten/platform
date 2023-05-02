use cosmwasm_std::{Addr, Binary, QueryRequest, Empty, Coin};
use schemars::JsonSchema;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};

/* 
pub const banka : QueryRequest<Empty> = QueryRequest::Bank( {
    cosmwasm_std::BankQuery::Balance { address: (), denom: () }
}) */

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Invest { amount: u128, validator: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TestQuery { request: QueryRequest<Empty> },
    InvestParams { address: Addr },
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentRewards {
    pub cw20_tokens: Vec<Addr>,
    pub native_tokens: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ValueQuerier {
    pub path: String,
    pub msg: QueryRequest<Empty>,
    pub group: u8,
    pub jq_parser: String,

}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeValue {
    Sender {},
    Contract {},
    TokenBalance {
        token: Token,
        amount: u128
    },
    Custom {
        value_type: String,
        value: Binary
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Attribute {
    pub key: String,
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
pub struct MessageBuilder {
    pub type_url: String,
    pub attributes: Vec<Attribute>
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestParamsResult {
    pub type_url: String,
    pub attributes: Vec<Attribute>
}





#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RewardToken {
    Contract { address : Addr },
    Native { denom : String },
    None {}
}


/* #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Strategy {
    investment_msg: InvestmentMsg,
    reward_token: RewardToken,
}

 */