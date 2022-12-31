use reqwest::{header, Client};
use std::error::Error;

const HEADERS: &str = "SlushPool-Auth-Token";
const HEADER_VALUE: &str = "<YOUR_SLUSHPOOL_API_KEY";

pub fn create_client() -> Result<Client, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(HEADERS, header::HeaderValue::from_static(HEADER_VALUE));

    let mut auth_value = header::HeaderValue::from_static(HEADER_VALUE);
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}

//#[cfg(test)]
//mod tests {
//    use reqwest::Client;
//    use std::error::Error;
//
//
//    extern crate reqwest;
//    extern crate tokio;
//    extern crate serde;
//
//
//    #[test]
//    fn test_create_client() -> Result<(), Box<dyn Error>> {
//
//        }
//}
