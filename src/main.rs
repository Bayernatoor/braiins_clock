#![allow(dead_code)]
mod client;
mod helpers;
mod requests;
mod run;

use crate::helpers::startup;
use crate::run::program_loop;

#[tokio::main]
async fn main() -> () {
    let intro_text = startup::introduction();
    println!("{}", intro_text);

    program_loop::program_loop().await;
}

