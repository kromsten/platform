use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


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
pub struct AutoStakeMsg {
    pub delegator_address: String,
    pub validator_address: String,
    pub enabled: bool
}




