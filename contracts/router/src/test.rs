#[cfg(test)]
mod tests {
    use crate::contract::{instantiate};
    use crate::error::ContractError;
    use crate::msg::{InstantiateMsg};

    use cosmwasm_std::{testing::*, MessageInfo, Response, DepsMut};
    use cosmwasm_std::{Coin, Uint128};


    pub fn default_info() -> MessageInfo {
        mock_info("admin",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        )
    }

    pub fn default_init(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let init_msg = InstantiateMsg { admin: None };
        instantiate(deps, mock_env(), info, init_msg)
    }


    #[test]
    fn can_instantiate() {
        let mut deps = mock_dependencies();
        let res: Result<Response, ContractError> = default_init(deps.as_mut(), default_info());
        assert!(res.is_ok());
        assert_eq!(0, res.unwrap().messages.len());
    }


    #[test]
    fn can_add_strategies() {
        let mut deps = mock_dependencies();

        let adming_info = default_info();

        default_init(deps.as_mut(), adming_info.clone()).unwrap();

        /* let msg = ExecuteMsg::AddRoute { address: Addr::unchecked("sscrt"), contract: todo!() };
        let res = execute(deps.as_mut(), mock_env(), mock_info("bob", &vec![]), msg);
        assert!(res.is_err());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::AllStrategies {}).unwrap();
        let strategies : Vec<Strategy> = from_binary(&res).unwrap();
        assert_eq!(0, strategies.len());

        let msg = ExecuteMsg::AddRoute { address: Addr::unchecked("sscrt") };
        let res = execute(deps.as_mut(), mock_env(), default_info(), msg);
        assert!(res.is_ok());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::AllStrategies {}).unwrap();
        let strategies : Vec<Strategy> = from_binary(&res).unwrap();
        assert_eq!(1, strategies.len()); */
    }

    #[test]
    fn can_add_transformations() {
        
    }

}