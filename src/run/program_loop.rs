use crate::helpers::startup;
use crate::requests::{self, send_to_blockclock::*};
use std::thread;
use std::time::Duration;

pub async fn program_loop() -> () {
    let mut tags = startup::select_tags();

    let refresh_rate = startup::refresh_rate();

    loop { 
        // grab next tag to display:
        let active_tag = &tags.remove(0);
        // add tag back to vec - infinite loop
        tags.push(active_tag.to_string());

        // get appropriate query for current tag. 
        let symbol = select_symbol(active_tag);

        
        let url_to_send = match active_tag.as_ref() {
            "confirmed_reward" | "all_time_reward" | "unconfirmed_reward" | "estimated_reward" | "off_workers" | "ok_workers" |
            "hash_rate_5m" | "hash_rate_60m" | "hash_rate_24h" | "hash_rate_scoring" => {
                requests::send_to_blockclock::create_slush_url(active_tag.to_string(), symbol).await
            }
            _ => requests::send_to_blockclock::create_blockclock_url(active_tag.to_string(), symbol).await,
        };
        
        let request = requests::send_to_blockclock::send_to_blockclock(url_to_send).await;
        
        match request.unwrap_or_else();

        println!("RESPONSE {:?}", request);
        
        println!("\nCurrently Displaying: {}\n", active_tag);

        let sleep_time = refresh_rate.as_ref().unwrap() * 60; 
        // let sleep_time = 60;
        println!("Sleeping for {} minutes", sleep_time / 60);
        thread::sleep(Duration::new(sleep_time as u64, 0));
    }
}
