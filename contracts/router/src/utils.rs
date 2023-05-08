use cosmwasm_std::{Addr, Deps, Decimal};
use secret_toolkit::utils::types::Token;

use crate::{error::ContractError, state::ADMIN, msg::TransformationFee};



pub fn check_admin (deps: Deps, sender: Addr) -> Result<bool, ContractError> {
    if ADMIN.load(deps.storage).unwrap() != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(true)
}

pub fn unwrap_token (token : &Token) -> (String, Option<String>) {
    match token {
        Token::Native (name)   => { (name.clone(), None)  },
        Token::Snip20 (contract) => { (contract.address.clone(), Some(contract.hash.clone())) }
    }
}


pub fn unwrap_fee (fee : &TransformationFee) -> Decimal {
    match fee {
        TransformationFee::Fixed { fee } => { fee.clone() },
        TransformationFee::Percentage { fee } => { fee.clone() }
        TransformationFee::QueryBuilder { builder: _} => { 
            panic!("Not for unwrapping!")
        }
    }
}
