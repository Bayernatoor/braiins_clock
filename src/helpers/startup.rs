use std::fmt::Debug;
use std::{io, error::Error}; 
use std::collections::HashMap;

pub fn introduction() -> String {
    let intro_text: String = String::from(" 
    <---------------------------->
    Select the tags you wish to display by entering the line number and pressing enter after each entry.\n
    The list will display all available tags. Slushpool tags and select Blockclock tags. 
    To end the selection process enter the character q and press enter.\n
    Tags will appear in the order selected and repeat continuously until the script is stopped (CTRL-C).
    You'll also need to select the refresh rate of the blockclock display. 
    Min: 5 minutes, Max: 10080 minutes (1 week, cause why not??!).
    
    This is a WORK IN PROGRESS. ENJOY :)   
    <---------------------------->
    ");
    return intro_text;
}

pub fn refresh_rate() -> Result<usize, Box<dyn Error>> {

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

        let rate: usize = match rate.trim().parse() {
            Ok(rate) => rate,
            Err(_) => continue,
        };
        loop_count += 1; 

        if range.contains(&rate) {
            println!("\nThe refresh rate is set to: {}mins", &rate);
            return Ok(rate);  
        } else {
            continue;
        }

    };
} 

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
      ("Sats per Dollar",  "cm.markets.sats_per_dollar"),
      ("Mempool Transactions", "cm.mempool.transactions"),
      ("Difficulty Retarget Date", "cm.retarget.retarget_date"),
      ("Blockchain Height", "cm.blockchain.block_height"),
      ("Moscow Time", "memes.moscow.time"),
    ]);

    println!("
    Please choose which tags you want to display");

    for (k, v) in tags.iter().enumerate() {
       println!("
       {:?} {:?}", k, v.0);
    };
    println!("");

    let mut selected_tags: Vec<String> = Vec::new();
    let mut selected = String::new(); 
    
    loop {
        selected.clear();

        io::stdin()
            .read_line(&mut selected)
            .expect("Could not parse line");
        
        if selected.trim() == "q" {
            println!("----------");
            break; 
        }

        let selected : usize = match selected.trim().parse() {
            Ok(selected) => selected,
            Err(_) => continue,  
        };

        let valid_ints = 0..=tags.len();
        if valid_ints.contains(&selected) {
            for (k, v) in tags.iter().enumerate() {
                if k == selected {
                    selected_tags.push(v.0.to_string());
                }
            }   
        } 

        //println!("The Vec contains: {:#?}", selected_tags);
        continue;
    }
    println!("The following tags will be displayed continuously, to stop press CTRL-C:");
    println!("{:#?}", selected_tags);
    println!("----------");
    return selected_tags;

}
