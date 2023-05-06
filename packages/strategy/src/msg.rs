use cosmwasm_std::{Addr, Binary, QueryRequest, Empty, Uint128, Timestamp};
use schemars::JsonSchema;
use secret_toolkit::utils::types::Token;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
    pub default_validator: Addr,
}



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {

    InvestParams {},
    WithdrawParams {},
    ClaimParams {},

    AprQuery {},
    RewardsQuery {},
    
    AllRewards { },
    Rewards { token: Token },
    Apr {},

    InvestMsgs {},
    WithdrawMsgs { address: Option<Addr> },
    ClaimMsgs { address: Option<Addr> },
    
    InvestTokens {},
    RewardTokens {},

    WithPermit {
        query: QueryRequest<Empty>,
    },


    TestQuery { 
        path: String,
        query: Binary
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
    CoinAmount {},
    ViewingKey {},
    
    String {
        value: String
    },

    Number {
        value: Uint128
    },

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
pub struct Reward {
    token: Token,
    amount: Uint128,
}

pub type IssuedToken = Reward;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionClass {
    CreateViewingKey {},
    SwapToken { from: Token, to: Token },
    Allowance { token: Token, amount: Uint128 },
    Mint { token: Token, amount: Uint128 },
    Transfer {},
    Staking {},
    Claiming {},
    Unknown {}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionRequrement {
    ViewingPermission {},
    Allowance { /* token: Token, amount: Uint128 */ },
    Authz { /* type_url: String, mesaage_builder: MessageBuilder */ },
    AdminRight {},
    Whitelist {},
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InvestmentAction {
    pub chain_id: String,
    pub type_url: String,
    pub attributes: Vec<Attribute>,
    pub exposes_investor: bool,
    pub issued_tokens: Option<Vec<IssuedToken>>,
    pub optional: bool,
    pub description: Option<String>,
    pub class: ActionClass,
    pub action_requirements: Option<Vec<ActionRequrement>>,
    pub independent_action_requirements: Option<Vec<ActionRequrement>>,
    pub unbonding: Option<Timestamp>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct MessageBuilder {
    pub name: String,
    pub attributes: Vec<Attribute>
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RequestBuilder {
    pub name: String,
    pub attributes: Vec<Attribute>
}

pub type InvestParamsResult = RequestBuilder;


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
