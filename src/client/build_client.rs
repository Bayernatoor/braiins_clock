use crate::helpers::env_vars::load_env_vars;
use reqwest::{header, Client};
use std::error::Error;

const HEADERS: &str = "SlushPool-Auth-Token";

pub async fn create_client() -> Result<Client, Box<dyn Error>> {
    // Load required SLUSHPOOL_API_KEY 
    let slushpool_api_key = load_env_vars("SLUSHPOOL_API_KEY");

    // additionals header values that will be added to our client
    let mut headers = header::HeaderMap::new();
    headers.insert(
        HEADERS,
        header::HeaderValue::from_str(&slushpool_api_key).unwrap(),
    );

    let mut auth_value = header::HeaderValue::from_str(&slushpool_api_key).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    // build the client and pass the result back to the caller. 
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
