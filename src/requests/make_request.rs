use crate::client::build_client;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug};

const SLUSH_POOL_URL: &str = "https://pool.braiins.com/accounts/profile/json/btc/";

//struct representing the response value from slushpool api - json response.
//unlikely to be the correct way to do this. if slushpools response changes this
// will break
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub username: String,
    pub btc: Btc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Btc {
    pub confirmed_reward: String,
    pub unconfirmed_reward: String,
    pub estimated_reward: String,
    pub all_time_reward: String,
    pub hash_rate_unit: String,
    pub hash_rate_5m: f64,
    pub hash_rate_60m: f64,
    pub hash_rate_24h: f64,
    pub hash_rate_scoring: f64,
    pub hash_rate_yesterday: f64,
    pub low_workers: f64,
    pub off_workers: f64,
    pub ok_workers: f64,
    pub dis_workers: f64,
}

//makes a GET request to slushpool's api and returns a Struct<Stats> Result.
pub async fn make_request() -> Result<Stats, Box<dyn Error>> {
    let client = build_client::create_client().await?;

    let response = client.get(SLUSH_POOL_URL).send().await?;

    let stats = response.json::<Stats>().await?;

    Ok(stats)
}
