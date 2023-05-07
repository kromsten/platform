use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use secret_toolkit::utils::types::{Contract, Token};
use serde::{Deserialize, Serialize};
use strategy::QueryBuilder;



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddStrategy { contract: Contract }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenStrategies { token : Token },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransformationFee {
    Fixed { fee: Decimal },
    Percentage { fee: Decimal },
    QueryBuilder { builder: QueryBuilder },
}