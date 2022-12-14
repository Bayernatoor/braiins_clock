#![allow(dead_code)]
mod requests;
mod client;
mod helpers;
mod run;

use std::error::Error;
use crate::requests::send_to_blockclock;
use crate::helpers::startup; 
use crate::run::program_loop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let intro_text=  startup::introduction(); 
    println!("{}", intro_text);

    program_loop::program_loop().await; 
    Ok(())
}

//1. Constants for URL and headers as well as blockclock IP
//2. HasHmap for available tags
//3. instructional text for user display in terminal
//4. Select desired tags and refresh rate
//5. and get to display a blockclock tag 
//6. send GET to display a custom slushpool tag 
//7. loop endlessly and display requeested tags 

// future features
// save original tags to a file and ask if user want to use those when rerunning program
// also ask if they want to overwrite it with the new tags
// perhaps they can save as many as they want and ask to display the lists.
//  saved_1, saved_2 ... show tags in each list. if they select a list we use that one. 
//
//
