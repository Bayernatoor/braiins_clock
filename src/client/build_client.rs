use std::env;
use std::error::Error;
use reqwest::{header, Client};
use crate::helpers::set_env_vars::set_env_vars;

const HEADERS: &str = "SlushPool-Auth-Token";

pub fn create_client() -> Result<Client, Box<dyn Error>>{
    let slushpool_api_key = set_env_vars("SLUSHPOOL_API_KEY");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        HEADERS, 
        header::HeaderValue::from_str(&slushpool_api_key).unwrap()
    );

    let mut auth_value = header::HeaderValue::from_str(&slushpool_api_key).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
    
}
