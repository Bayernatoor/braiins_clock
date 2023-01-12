use crate::helpers::startup;
use crate::requests::{self, send_to_blockclock::*};
use hyper;
use std::thread;
use std::time::Duration;


// loops through user selected tags and is responsbile for calling all helper functions
// Engine that runs the program. Called in main. 
pub async fn program_loop() -> () {

    // set startup functions 
    let mut tags = startup::select_tags();
    let refresh_rate = startup::refresh_rate();

    loop {
        // grab next tag to display:
        let active_tag = &tags.remove(0);
        // add tag back to vec - infinite loop
        tags.push(active_tag.to_string());

        // get appropriate query for current tag.
        let symbol = select_symbol(active_tag);

        // determines which url builder to use based on tag selected
        // either slush pool url or blocklock url
        let url_to_send = match active_tag.as_ref() {
            "confirmed_reward" | "all_time_reward" | "unconfirmed_reward" | "estimated_reward"
            | "off_workers" | "ok_workers" | "hash_rate_5m" | "hash_rate_60m" | "hash_rate_24h"
            | "hash_rate_scoring" => {
                requests::send_to_blockclock::create_slush_url(active_tag.to_string(), symbol).await
            }
            _ => {
                requests::send_to_blockclock::create_blockclock_url(active_tag.to_string(), symbol)
                    .await
            }
        };

        // makes a GET request to the blockclock with the appropriate url. 
        let request = requests::send_to_blockclock::send_to_blockclock(url_to_send).await;
        match request {
            Ok(response) => {
                println!("Blockblock received request successfully: {:?}:", response);
            }
            Err(error) => {
                if let Some(_hyper_error) = error.downcast_ref::<hyper::Error>() {
                    continue;
                } else {
                    println!("Error while dispatching to blocklock, you may want to check your clock's IP address: {error}");
                    println!("\nTrying again in 10 seconds");
                    thread::sleep(Duration::new(10, 0));
                    continue;
                }
            }
        };

        println!("Currently Displaying: {active_tag}\n");
        // refresh rate of display is used to set thread::sleep() 
        let sleep_time = refresh_rate.as_ref().unwrap() * 60;
        println!("Sleeping for {} minutes", sleep_time / 60);
        thread::sleep(Duration::new(sleep_time as u64, 0));
    }
}
