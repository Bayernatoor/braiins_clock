use reqwest::Response;

use crate::client::build_client;
use crate::helpers::env_vars::load_env_vars;
use crate::requests::make_request;
use std::{error::Error, fmt::Debug};

// struct to represent an URL
#[derive(Debug)]
pub struct URL<'a> {
    protocol: &'a str,
    domain: String,
    path: &'a str,
    result: String,
    query: String,
}

impl<'a> URL<'a> {
    // creates an instance of Struct URL with some pre set values 
    // requires 3 vars as well. 
    fn new_url(path: &'a str, result: String, query: String) -> URL<'a> {
        let blockclock_ip = load_env_vars("BLOCKCLOCK_IP");
        URL {
            protocol: "http://",
            domain: { blockclock_ip },
            path,
            result,
            query,
        }
    }

    // use format! macro to build the URL into a String type
    fn build_url(&self) -> String {
        return format!(
            "{}{}{}{}{}",
            self.protocol, self.domain, self.path, self.result, self.query
        );
    }

    // use format! macro to build the URL into a String type - does not take query param. 
    fn build_blockclock_url(&self) -> String {
        return format!(
            "{}{}{}{}",
            self.protocol, self.domain, self.path, self.result
        );
    }
}

// matches the selected tag with the appropriate symbol for url construction
pub fn select_symbol(tag: &str) -> String {
    let symbol = match tag {
        "confirmed_reward" | "unconfirmed_reward" | "estimated_reward" | "all_time_reward" => {
            String::from("?sym=bitcoin")
        }
        "off_workers" => String::from("?pair=ASIC/DOWN"),
        "ok_workers" => String::from("?pair=ASIC/UP"),
        "hash_rate_5m" | "hash_rate_60m" | "hash_rate_24h" | "hash_rate_scoring" => {
            String::from("?pair=TH/S")
        }
        _ => String::from("?sym=bitcoin"),
    };
    return symbol;
}

pub fn select_tiny_text(_tag: &str) -> String {
    todo!()
}

// takes a tag matches on appropriate value in struct
// returns its f64 value after making a request to slushpool api
pub async fn get_slushpool_stats(tag: &str) -> Result<f64, Box<dyn Error>> {
    // Handle potential errors
    let stats = make_request::make_request().await?;

    // return f64 value of selected tag - Strings are converted to floats.
    let to_float = match tag.as_ref() {
        "confirmed_reward" => stats.btc.confirmed_reward.parse::<f64>(),
        "unconfirmed_reward" => stats.btc.unconfirmed_reward.parse::<f64>(),
        "estimated_reward" => stats.btc.estimated_reward.parse::<f64>(),
        "all_time_reward" => stats.btc.all_time_reward.parse::<f64>(),
        "hash_rate_unit" => stats.btc.hash_rate_unit.parse::<f64>(),
        "hash_rate_5m" => Ok(stats.btc.hash_rate_5m),
        "hash_rate_60m" => Ok(stats.btc.hash_rate_60m),
        "hash_rate_24h" => Ok(stats.btc.hash_rate_24h),
        "hash_rate_scoring" => Ok(stats.btc.hash_rate_scoring),
        "hash_rate_yesterday" => Ok(stats.btc.hash_rate_yesterday),
        "low_workers" => Ok(stats.btc.low_workers),
        "off_workers" => Ok(stats.btc.off_workers),
        "ok_workers" => Ok(stats.btc.ok_workers),
        "dis_workers" => Ok(stats.btc.dis_workers),
        _ => String::from("0.0").parse::<f64>(),
    };

    Ok(to_float?)
}

/*
Instantiates a reqwest client and takes a URL as parameter 
to make GET request to blocklock. 
*/
pub async fn send_to_blockclock(url: String) -> Result<Response, Box<dyn Error>> {
    loop {
        let client = build_client::create_client().await;
        match client {
            Ok(client) => {
                // if no errors with client return a Result with OK value to caller. 
                let dispatch_to_blockclock = client.get(url).send().await?;
                return Ok(dispatch_to_blockclock);
            }
            //Any error we handle by trying again
            //this needs work
            Err(error) => {
                if let Some(_hyper_error) = error.downcast_ref::<hyper::Error>() {
                    continue;
                } else {
                    eprintln!("Error while creating client: {error}");
                    println!("Trying again...please standby");
                    continue;
                }
            }
        };
    }
}

// Build the slushpool url String
// Also formats the values used in the URL query portion to ensure they
// look good on blockclock. - may split this off into it's func later. 
pub async fn create_slush_url(tag: String, mut query: String) -> String {
    let result = match get_slushpool_stats(&tag).await {
        Ok(result) => result.to_string(),
        Err(_error) => {
            eprintln!("Error parsing result, defaulting to 0");
            "0".to_string()
        }
    };

    println!("RESULT from create_slush_url {result}");

    let mut result_splice = String::new();

    if tag.contains("hash") {
        let hash_value = result[0..7].to_string();
        // formatted to represent terahash/second. 
        let hash_formatted = hash_value.parse::<f64>().unwrap() / 1000.0;
        result_splice = hash_formatted.round().to_string();
    } else if tag.contains("reward") {
        // may want to fix this to return less decimals,
        // scientific notation kicks in at 5 decimal points.
        result_splice = result[0..7].to_string();
    } else if tag.contains("worker") {
        result_splice = result;
    } else {
        // any error will display 0 
        result_splice = "0".to_string();
        query = "?pair=N/A".to_string();
    }

    let url = URL::new_url("/api/show/number/", result_splice.to_string(), query).build_url();
    return url;
}

// blockclock url is simple since it handles all tiny text and decoration itself. 
pub async fn create_blockclock_url(result: String, query: String) -> String {
    let url = URL::new_url("/api/pick/", result, query).build_blockclock_url();
    return url;
}
