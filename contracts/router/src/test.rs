#[cfg(test)]
mod tests {
    use crate::contract::{instantiate, execute, query};
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
    use crate::state::Strategy;

    use cosmwasm_std::{testing::*, MessageInfo, Response, StdResult, StdError, Addr};
    use cosmwasm_std::{from_binary, Coin, Uint128};



    pub fn default_info() -> MessageInfo {
        mock_info("admin",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        )
    }

    pub fn default_init() -> StdResult<Response> {
        let mut deps = mock_dependencies();
        let info: MessageInfo = default_info();
        let init_msg = InstantiateMsg { admin: None };
        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, init_msg)
    }

    #[test]
    fn can_instantiate() {
        let res: Result<Response, StdError> = default_init();
        assert!(res.is_ok());
        assert_eq!(0, res.unwrap().messages.len());
    }

    #[test]
    fn can_add_strategies() {
        default_init().unwrap();
        
        let mut deps = mock_dependencies();
        
        let msg = ExecuteMsg::AddRoute { address: Addr::unchecked("sscrt") };
        let res = execute(deps.as_mut(), mock_env(), default_info(), msg);
        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::AllStrategies {});
        assert!(res.is_ok());

        let strategies : Vec<Strategy> = from_binary(&res.unwrap()).unwrap();
        assert_eq!(1, strategies.len());
    }

}