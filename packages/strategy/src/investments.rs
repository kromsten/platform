
use crate::{
    msg::{InvestmentAction, ActionClass, ActionRequrement}, 
};


pub fn default_msg() -> InvestmentAction {
    InvestmentAction { 
        chain_id: "secret-4".to_string(), 
        type_url: String::new(), 
        attributes: Vec::new(), 
        exposes_investor: true, 
        issued_tokens: None, 
        optional: false,
        description: None,
        class: ActionClass::Unknown {},
        action_requirements: None,
        independent_action_requirements: Some(vec![ActionRequrement::Authz {}]),
        unbonding: None,
    }
}

