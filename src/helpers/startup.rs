use std::collections::HashMap;
use std::{error::Error, io};

// it's just an intro text!
pub fn introduction() -> String {
    let intro_text: String = String::from(" 
    WELCOME BITCOINER - Here's a quick intro:  
                                          
    <---------------------------->
    Select the tags you wish to display by entering the line number and pressing enter after each entry.\n
    The list will display all available tags. Braiins Pool tags and certain Blockclock tags. 
    To end the selection process enter the character \"q\" and press enter.\n
    Tags will appear in the order selected and repeat continuously until the script is stopped, to stop press --> (CTRL-C).
    You'll also need to select the refresh rate of the blockclock display. 
    Min: 5 minutes, Max: 10080 minutes (1 week, cause why not??!).
    
    This is a WORK IN PROGRESS. ENJOY :)   
    <---------------------------->
    ");
    return intro_text;
}

// Ask user for input to set refresh rate of blockclock display in minutes
pub fn refresh_rate() -> Result<i32, Box<dyn Error>> {
    println!("Please select a refresh rate -- Any number between 5-10080");

    let mut loop_count = 0;
    loop {
        let mut rate = String::new();
        let range = 5..10080;

        if loop_count > 0 {
            println!("You must enter a number between 5-10080")
        };

        io::stdin()
            .read_line(&mut rate)
            .expect("Failed to read line");

        let rate: i32 = match rate.trim().parse() {
            Ok(rate) => rate,
            Err(_) => continue,
        };
        loop_count += 1;

        if range.contains(&rate) {
            println!("\nThe refresh rate is set to: {}mins\n", &rate);
            return Ok(rate);
        } else {
            continue;
        }
    }
}

// display available tags to the user and add them to a Vec
pub fn select_tags() -> Vec<String> {
    let tags = HashMap::from([
        ("Confirmed Reward", "confirmed_reward"),
        ("Unconfirmed Reward", "unconfirmed_reward"),
        ("Estimated Reward", "estimated_reward"),
        ("Alltime Reward", "all_time_reward"),
        ("Hashrate 5m", "hash_rate_5m"),
        ("Hashrate 60m", "hash_rate_60m"),
        ("Hashrate 24h", "hash_rate_24h"),
        ("Hashrate Scoring", "hash_rate_scoring"),
        ("Active Workers", "ok_workers"),
        ("Offline workers", "off_workers"),
        ("Estimated Hash Rate", "cm.mining.hash_rate_2016_blocks"),
        ("USD Market Price", "cm.markets.price"),
        ("EUR Market Price", "coinbase.BTC-EUR.spot"),
        ("GBP Market Price", "coinbase.BTC-GBP.spot"),
        ("Sats per Dollar", "cm.markets.sats_per_dollar"),
        ("Mempool Transactions", "cm.mempool.transactions"),
        ("Difficulty Retarget Date", "cm.retarget.retarget_date"),
        ("Blockchain Height", "cm.blockchain.block_height"),
        ("Moscow Time", "memes.moscow.time"),
    ]);

    println!(
        "
    Please choose which tags you want to display"
    );

    // displays tags to user - 0 "tag", 1 "tag", etc..
    for (k, v) in tags.iter().enumerate() {
        println!(
            "
       {:?} {:?}",
            k, v.0
        );
    }
    println!("\n");

    // vec returned by function - vlaues in tags HashMap
    let mut selected_tags: Vec<String> = Vec::new();
    // vec used to displayed tags selected by user - key in tags HashMap
    let mut displayed_tags: Vec<String> = Vec::new();
    let mut selected = String::new();

    // accepts int values representing the tags - runs indefinitely until user
    // types the character "q".
    loop {
        selected.clear();

        io::stdin()
            .read_line(&mut selected)
            .expect("Could not parse line");

        // if q is entered break out of loop and continue program.
        if selected.trim() == "q" {
            println!("----------");
            break;
        }

        // make sure it's an int and strip whitepace
        let selected: usize = match selected.trim().parse() {
            Ok(selected) => selected,
            Err(_) => continue,
        };

        // make sure the int provided corresponds to a tag
        // if not reject it.
        let valid_ints = 0..=tags.len();
        if valid_ints.contains(&selected) {
            for (k, v) in tags.iter().enumerate() {
                if k == selected {
                    selected_tags.push(v.1.to_string());
                    displayed_tags.push(v.0.to_string());
                }
            }
        }
        continue;
    }
    println!("The following tags will be displayed continuously, to stop press CTRL-C:");
    println!("{displayed_tags:#?}");
    println!("----------");
    return selected_tags;
}
