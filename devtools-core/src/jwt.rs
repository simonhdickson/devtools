use std::collections::BTreeMap;

use jwt::{Header, Unverified};
use thiserror::Error;

#[derive(Default)]
pub struct Jwt {
    jwt_token: String,
    headers: String,
    payload: String,
}

#[derive(Error, Debug)]
pub enum SetTokenError {
    #[error("failed to parse jwt token")]
    Parse(#[from] jwt::error::Error),
    #[error("failed to parse token headers")]
    Headers(serde_json::error::Error),
    #[error("failed to parse token payload")]
    Payload(serde_json::error::Error),
}

impl Jwt {
    pub fn set_token_string(&mut self, s: &str) -> Result<(), SetTokenError> {
        self.jwt_token = s.to_owned();

        let t = jwt::Token::<Header, BTreeMap<String, serde_json::Value>, Unverified>::parse_unverified(
            &*self.jwt_token,
        )?;

        self.headers = serde_json::to_string_pretty(&t.header()).map_err(SetTokenError::Headers)?;
        self.payload = serde_json::to_string_pretty(&t.claims()).map_err(SetTokenError::Payload)?;

        Ok(())
    }

    pub fn get_token_string(&self) -> &str {
        &self.jwt_token
    }

    pub fn get_headers(&self) -> &str {
        &self.headers
    }

    pub fn get_payload(&self) -> &str {
        &self.payload
    }
}
