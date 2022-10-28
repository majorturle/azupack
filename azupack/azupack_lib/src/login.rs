use std::{env, fmt, error::Error};
use error_stack::{IntoReport, Result};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LoginToken {
    pub organization: String,
    pub token: String
}

pub struct LoginProvider {
    tokens: Vec<LoginToken>
}

impl LoginToken {
    pub fn new<S> (organization: S, token: S) -> LoginToken where S: Into<String> {
        let mut org: String = organization.into();
        if org.chars().last().unwrap() != '/' {
            org.push('/');
        }

        LoginToken { organization: org, token: token.into() }
    }

    pub fn is_valid(&self) -> bool {
        // todo: make a regex match to check if it's an URL and a real token
        if self.organization.len() > 0 && self.token.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
}


#[derive(Debug)]
pub struct LoginNotFoundError;

impl fmt::Display for LoginNotFoundError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Azure CLI login: login instance not found.")
    }
}

impl Error for LoginNotFoundError {}

impl LoginProvider {
    pub fn new () -> LoginProvider {
        // fetch the tokens from the environment
        let mut login_map: HashMap<i32, LoginToken> = HashMap::new();
        for (key, value) in env::vars() {
            // try to capture the organizations AZUPACK_LOGIN__0__ORG = ""
            let re = Regex::new(r"AZUPACK_LOGIN__(\d+)__ORG").unwrap();
            if let Some(caps) = re.captures(&key) {
                let org_index = caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                let org_name = value;
                login_map.insert(org_index, LoginToken::new(org_name, String::new()));
            } else {
                // try to capture the tokens AZUPACK_LOGIN__0__TOKEN = ""
                let re = Regex::new(r"AZUPACK_LOGIN__(\d+)__TOKEN").unwrap();
                if let Some(caps) = re.captures(&key) {
                    let token_index = caps.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                    let token_value = value;

                    // check if the organization is already in the dictionary
                    if login_map.contains_key(&token_index) {
                        if let Some(x) = login_map.get_mut(&token_index) {
                            x.token = token_value;
                        }
                    }
                }
            }
        }

        let mut provider = LoginProvider { tokens: Vec::new() };
        for (_, token) in login_map {
            if token.is_valid() {
                provider.add_token(token);
            }
        }

        return provider;
    }

    pub fn add_token(&mut self, token: LoginToken) {
        self.tokens.push(token)
    }

    pub fn get_tokens<S>(&self, organization: S) -> Result<Vec<&LoginToken>, LoginNotFoundError> where S: Into<String> {
        let mut org: String = organization.into();
        if org.chars().last().unwrap() != '/' {
            org.push('/');
        }

        let result = self.tokens.iter()
            .filter(|x| x.organization == org)
            .collect::<Vec<_>>();
        if !result.is_empty() {
            Ok(result)
        } else {
            Err(LoginNotFoundError).into_report()
        }
    }
}