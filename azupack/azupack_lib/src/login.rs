use std::{fmt, error::Error};
use error_stack::{IntoReport, Result};

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
        LoginProvider { tokens: Vec::new() }
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