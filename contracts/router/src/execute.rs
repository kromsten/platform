use cosmwasm_std::{
    DepsMut, 
    Addr, 
    StdResult, 
    StdError, 
    Response, 
    WasmQuery, 
    QueryRequest, 
    Empty, 
    to_binary, Event, Attribute, 
};
use secret_toolkit::utils::{types::{Contract, Token}};
use crate::{state::{ADMIN, TokenStrategy, STRATEGY_ROUTER}, utils::unwrap_token};

use strategy::{
    QueryMsg::{InvestTokens, RewardTokens, InvestMsgs, InvestParams}, 
    InvestmentAction, InvestParamsResult
};



pub fn add_route(
    deps: DepsMut,
    sender: Addr,
    contract: Contract,
) -> StdResult<Response> {

    if ADMIN.load(deps.storage).unwrap() != sender {
        return Err(StdError::GenericErr { msg: "Only admin can add routes".to_string() });
    }


    let inputs_tokens : Vec<Token> = deps.querier.query(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart { 
        contract_addr: contract.address.clone(), 
        code_hash: contract.hash.clone(), 
        msg: to_binary(&InvestTokens {})?
    }))?;


    deps.api.debug(format!("Invest tokens: {:?}", inputs_tokens).as_str());


    let output_tokens : Vec<Token> = deps.querier.query(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart { 
        contract_addr: contract.address.clone(), 
        code_hash: contract.hash.clone(), 
        msg: to_binary(&RewardTokens{})?
    }))?;


    deps.api.debug(format!("Rewards tokens: {:?}", output_tokens.clone()).as_str());

    let actions : Vec<InvestmentAction> = deps.querier.query(&QueryRequest::<Empty>::Wasm(
        WasmQuery::Smart { 
            contract_addr: contract.address.clone(),
            code_hash: contract.hash.clone(), 
            msg: to_binary(&InvestMsgs {})? 
        }
    ))?;


    deps.api.debug(format!("Investment actions: {:?}", actions).as_str());


    let params : InvestParamsResult = deps.querier.query(&QueryRequest::<Empty>::Wasm(
        WasmQuery::Smart { 
            contract_addr: contract.address.clone(),
            code_hash: contract.hash.clone(), 
            msg: to_binary(&InvestParams {})? 
        }
    ))?;

    deps.api.debug(format!("Investment params: {:?}", params).as_str());

    let mut attributes : Vec<Attribute> = Vec::with_capacity(inputs_tokens.len());

    for token in inputs_tokens.clone() {

        let token_name = unwrap_token(&token).0;
        
        STRATEGY_ROUTER
        .add_suffix(token_name.as_bytes())
        .insert(deps.storage, &contract.address, &TokenStrategy {
            inputs: inputs_tokens.clone(),
            outputs: output_tokens.clone(),
        })?;

        attributes.push(
            Attribute { key: "token_name".to_owned(), value: token_name, encrypted: false }
        )
    }

    Ok(Response::default()
        .add_event(
            Event::new(String::from("added_strategy"))
            .add_attributes(attributes)
        )
    )
}