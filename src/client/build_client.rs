use std::error::Error;
use reqwest::{header, Client};
use crate::helpers::env_vars::load_env_vars;

const HEADERS: &str = "SlushPool-Auth-Token";

pub async fn create_client() -> Result<Client, Box<dyn Error>>{
    let slushpool_api_key = load_env_vars("SLUSHPOOL_API_KEY");

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
