use cosmwasm_std::{CosmosMsg, Addr, Binary, QueryRequest, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/* 
pub const banka : QueryRequest<Empty> = QueryRequest::Bank( {
    cosmwasm_std::BankQuery::Balance { address: (), denom: () }
}) */

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    DefineRoute {}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TestQuery { request: QueryRequest<Empty> },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentRewards {
    pub cw20_tokens: Vec<Addr>,
    pub native_tokens: Vec<String>,
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ValueQuerier {
    pub msg: QueryRequest<Empty>,
    pub group: u8,
    pub jq_parser: String,

}


/* #[non_exhaustive] */



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeValue {
    Sender {},
    Contract {},
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
pub struct MessageBuilder {
    pub type_url: String,
    pub attributes: Vec<Attribute>
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentMsg {
    msg: Option<CosmosMsg>,
    builder: Option<MessageBuilder>,
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RewardToken {
    Contract { address : Addr },
    Native { denom : String },
    None {}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Strategy {
    investment_msg: InvestmentMsg,
    reward_token: RewardToken,
}

