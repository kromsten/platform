use cosmwasm_std::{
    DepsMut, 
    Addr, 
    Response, 
    WasmQuery, 
    QueryRequest, 
    Empty, 
    to_binary, Event, Attribute, Decimal, 
};

use secret_toolkit::utils::{types::{Contract, Token}};
use crate::{
    state::{TokenStrategy, STRATEGY_ROUTER, TRANSFORMATIONS, Transformation}, 
    utils::{unwrap_token, check_admin}, 
    msg::TransformationFee, 
    error::ContractError
};

use strategy::{
    QueryMsg::{InvestTokens, RewardTokens, InvestMsgs, InvestParams}, 
    InvestmentAction, InvestParamsResult
};



pub fn add_strategy(
    deps: DepsMut,
    sender: Addr,
    contract: Contract,
) -> Result<Response, ContractError> {

    check_admin(deps.as_ref(), sender)?;

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
        
        let prefixed =STRATEGY_ROUTER.add_suffix(token_name.as_bytes());

        if prefixed.iter(deps.storage)?.any(|strategy| 
            strategy.unwrap().contract.address == contract.address
        ) {
            return Err(ContractError::StrategyAlreadyExists {})
        }


        prefixed.push(deps.storage,  &TokenStrategy {
            contract: contract.clone(),
            inputs: inputs_tokens.clone(),
            outputs: output_tokens.clone(),
            apr: Decimal::zero()
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



pub fn add_transformation(
    deps: DepsMut, 
    sender: Addr,
    token_in : Token, 
    token_out: Token,
    actions: InvestmentAction,
    fee: TransformationFee
) -> Result<Response, ContractError> {

    check_admin(deps.as_ref(), sender)?;
    
    let name_in = unwrap_token(&token_in).0;
    let name_out = unwrap_token(&token_out).0;


    TRANSFORMATIONS
        .add_suffix(name_in.as_bytes())
        .add_suffix(name_out.as_bytes())
        .push(deps.storage, &Transformation {
            actions,
            fee,
    })?;

    Ok(Response::default())
}