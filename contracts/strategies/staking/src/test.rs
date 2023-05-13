#[cfg(test)]
mod tests {
    use strategy::{InvestParamsResult, Attribute, QueryMsg};
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::contract::{instantiate, query, execute};

    use cosmwasm_std::{testing::*, Api, OwnedDeps, MemoryStorage, Addr};
    use cosmwasm_std::{from_binary, Coin, Uint128};

    fn attribute_checker(
        deps : &OwnedDeps<MemoryStorage, MockApi, MockQuerier>, 
        attr: Attribute
    )  {

        if attr.querier.is_some() {
            let q = attr.querier.unwrap();

            let res = deps.querier.handle_query(&q.request).unwrap();

            deps.api.debug(&format!("res: {:?}", res));

        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let init_msg = InstantiateMsg { admin: None, default_validator: Addr::unchecked("validator") };
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        assert_eq!(0, res.messages.len());
    }




    #[test]
    fn query_invest_params() {
        let mut deps = mock_dependencies();

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { 
            admin: None, 
            default_validator: Addr::unchecked("validator")};

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();


        let qmsg : QueryMsg = QueryMsg::InvestParams {  };

        let result = query(deps.as_ref(), mock_env(), qmsg);

        assert_eq!(result.is_ok(), true);

        let response : InvestParamsResult = from_binary(&result.unwrap()).unwrap();

        deps.api.debug(&format!("invest params res: {:?}", response));

        response.attributes
            .iter()
            .for_each(|attr| attribute_checker(&deps, attr.clone()));


    }


    #[test]
    fn invest() {
        let mut deps = mock_dependencies();

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { admin: None, default_validator: Addr::unchecked("validator")};
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();



        let invest_msg = ExecuteMsg::Invest { 
            amount: Uint128::new(2), 
            validator_address: Some(String::from("validator")),
            delegator_address: Some(String::from("delegator")),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), invest_msg);


        println!("ex res: {:?}", res);

        assert!(res.is_ok());

        assert_eq!(1, 2)


    }

}