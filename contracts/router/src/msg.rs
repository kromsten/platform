use cosmwasm_std::{CosmosMsg, Addr, Binary, QueryRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20::{Balance, Cw20CoinVerified};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
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
pub struct CustomQuery {
    raw: Binary,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ValueQuerier {
    pub msg: QueryRequest<CustomQuery>,
    pub group: u8,
    pub jq_parser: String,

}



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
pub struct MessageBuilder {
    pub type_url: String,
    pub attributes: Vec<Attribute>
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentMsg {
    msg: Option<CosmosMsg>,
    builder: Option<MessageBuilder>,
}

