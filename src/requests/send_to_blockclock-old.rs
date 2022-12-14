use crate::requests::make_request;
use crate::client::build_client;
use std::{error::Error, num::ParseFloatError, fmt::{Debug, Result}};

const BLOCKCLOCK_IP: &str = "192.168.1.15";

#[derive(Debug)]
pub struct URL<'a> {
    protocol: &'a str,
    domain: &'a str,
    path: &'a str,
    result: String,
    query: Option<String>,
}

impl<'a> URL<'a> {

    fn new_slush_url(path: &'a str, result: String, query: Option<String>) -> URL<'a> {
        URL { protocol: "http://", domain: {BLOCKCLOCK_IP}, path, result, query}
    } 
    
    fn new_blockclock_url(path: &'a str, result: String, query: Option<String>) -> URL<'a> {
        URL { protocol: "http://", domain: {BLOCKCLOCK_IP}, path, result, query}
    } 
    
    fn build_url(&self) -> String {
       return format!("{}{}{}{}{:?}", self.protocol, self.domain, self.path, self.result, self.query);
    }

    //fn to_float(&self) -> Result<f64, ParseFloatError> {
    //    return self.result.parse::<f64>().unwrap.expect("Could not parse to float");
    //}

}

// matches the selected tag with the appropriate symbol for url construction 
fn select_symbol(t: &str) -> String {
       let symbol =  match t {
            "confirmed_reward" | "unconfirmed_reward" |  "estimated_reward" | "alltime_reward" => String::from("?pair=bitcoin"),
            "off_workers" => String::from("?pair=ASIC/UP"),
            "ok_workers" => String::from("?pair=ASIC/UP"),
            "hash_rate_5m" | "hash_rate_60m" |  "hash_rate_24h" | "hash_rate_scoring" => String::from("?pair=TH/S"),
            _ => String::from("pair=bitcoin"),
        };
        return symbol 

}

pub async fn convert_to_float(tag: String) -> Result<f64, > {
    let stats = make_request::make_request()
        .await?;

    let to_float = match tag.as_ref() {
        "confirmed_reward" => stats.btc.confirmed_reward.parse::<f64>(),
        "unconfirmed_reward" => stats.btc.unconfirmed_reward.parse::<f64>(),
        "estimated_reward" => stats.btc.estimated_reward.parse::<f64>(),
        "all_time_reward" => stats.btc.all_time_reward.parse::<f64>(),
        "hash_rate_unit" => stats.btc.hash_rate_unit.parse::<f64>(),
        //"hash_rate_5m" => Ok(stats.btc.hash_rate_5m),
        //"hash_rate_60m" => Ok(stats.btc.hash_rate_60m),
        //"hash_rate_24h" => Ok(stats.btc.hash_rate_24h),
        //"hash_rate_scoring" => Ok(stats.btc.hash_rate_scoring),
        //"hash_rate_yesterday" => Ok(stats.btc.hash_rate_yesterday),
        //"low_workers" => Ok(stats.btc.low_workers),
        //"off_workers" => Ok(stats.btc.off_workers),
        //"ok_workers" => Ok(stats.btc.ok_workers),
        //"dis_workers" => Ok(stats.btc.dis_workers),
    };

    Ok(())
}


pub async fn send_to_blockclock(url: String) -> Result<(), Box<dyn Error>> { 
    let block_client = build_client::create_client();
    let send_blockclock = block_client?
        .get(url)
        .send()
        .await?;
    
    Ok(())
}

pub async fn slush_tags_url(tag: f64, symbol: Option<String>) -> String {
     
    let result = tag.to_string();

    let url = URL::new_slush_url("/api/show/number/", result, symbol).build_url();
    return url
}

pub async fn clock_tags_url(tag: String, symbol: Option<String>) -> String {
    let url = URL::new_blockclock_url("/api/pick/", tag, symbol).build_url();
    return url
}


