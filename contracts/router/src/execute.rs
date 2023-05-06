use cosmwasm_std::{DepsMut, Addr, StdResult, StdError, Response, WasmQuery, to_binary, WasmMsg, QueryRequest, Empty, Binary, from_binary};
use secret_toolkit::utils::{types::{Contract, Token}, Query};
use crate::{state::{STRATEGY_ROUTER, Strategy, ADMIN}, contract, msg::StrategyQueryMsg};




pub fn add_route(
    deps: DepsMut,
    sender: Addr,
    contract: Contract,
) -> StdResult<Response> {

    if ADMIN.load(deps.storage).unwrap() != sender {
        return Err(StdError::GenericErr { msg: "Only admin can add routes".to_string() });
    }


    let tokens : Vec<Token> = deps.querier.query(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart { 
        contract_addr: contract.address.clone(), 
        code_hash: contract.hash.clone(), 
        msg: to_binary(&StrategyQueryMsg::InvestTokens {})?
    }))?;


    deps.api.debug(format!("Invest tokens: {:?}", tokens).as_str());


    let tokens : Vec<Token> = deps.querier.query(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart { 
        contract_addr: contract.address.clone(), 
        code_hash: contract.hash, 
        msg: to_binary(&StrategyQueryMsg::RewardTokens{})?
    }))?;


    deps.api.debug(format!("Rewards tokens: {:?}", tokens).as_str());
    

    // STRATEGY_ROUTER.insert(deps.storage, &addr, &Strategy {}).unwrap();

    Ok(Response::default())
}