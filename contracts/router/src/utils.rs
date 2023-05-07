use secret_toolkit::utils::types::Token;

pub fn unwrap_token (token : &Token) -> (String, Option<String>) {
    match token {
        Token::Native (name)   => { (name.clone(), None)  },
        Token::Snip20 (contract) => { (contract.address.clone(), Some(contract.hash.clone())) }
    }
}
