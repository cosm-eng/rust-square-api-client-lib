use std::{collections::HashMap, env};

use log::{error, warn};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

use crate::models::errors::ApiError;

use super::client::HttpClientConfiguration;

const DEFAULT_SQUARE_VERSION: &str = "2022-02-16";

#[derive(Clone, Debug)]
pub struct Headers {
    pub headers: HashMap<String, String>,
}

impl Headers {
    pub fn has_user_agent(&self) -> bool {
        self.headers.contains_key("user-agent")
    }

    pub fn set_user_agent(&mut self, user_agent: &str) -> Option<String> {
        self.insert("user-agent", user_agent)
    }

    pub fn insert(&mut self, header_name: &str, header_value: &str) -> Option<String> {
        self.headers.insert(String::from(header_name), String::from(header_value))
    }

    pub(crate) fn default_authorization() -> String {
        format!("Bearer {}", env::var("SQUARE_API_TOKEN").unwrap_or_else(|_| {
            warn!("No SQUARE_API_TOKEN environment variable found");
            String::new()
        }))
    }
}

impl Default for Headers {
    fn default() -> Self {
        let mut headers = HashMap::new();

        headers.insert(String::from("Content-Type"), String::from("application/json"));
        headers.insert(String::from("Square-Version"), String::from(DEFAULT_SQUARE_VERSION));
        headers.insert(String::from("accept"), String::from("application/json"));
        headers.insert(String::from("user-agent"), HttpClientConfiguration::default_user_agent());
        headers.insert(String::from("Authorization"), Self::default_authorization());

        Self { headers }
    }
}

impl TryFrom<&Headers> for HeaderMap {
    type Error = ApiError;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        let mut header_map = Self::new();
        for (k, v) in &headers.headers {
            let header_name = HeaderName::from_bytes(k.as_bytes()).map_err(|e| {
                let msg = format!("Error generating {} header name: {}", k, e);
                error!("{}", msg);
                ApiError::new(&msg)
            })?;
            let header_value = HeaderValue::from_bytes(v.as_bytes()).map_err(|e| {
                let msg = format!("Error generating {} header value for header {}: {}", v, k, e);
                error!("{}", msg);
                ApiError::new(&msg)
            })?;
            header_map.insert(header_name, header_value);
        }

        Ok(header_map)
    }
}
