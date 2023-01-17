#![allow(dead_code)]

mod client;
mod helpers;
mod requests;
mod run;

use std::io;

use helpers::env_vars;

use crate::helpers::startup;
use crate::run::program_loop;

#[tokio::main]
async fn main() {
    // lets check if our env_vars are set if true program starts running
    // otherwise let's ask user to set them and try again.
    let env_vars = helpers::env_vars::check_env_var_exists();

    if env_vars {
        let mut reset_vars = String::new();
        println!("\nWould you like to change your environment variables? Enter y/n");

        match io::stdin().read_line(&mut reset_vars) {
            Ok(_n) => println!("............. \n"),
            Err(error) => println!("Error reading line: {error}"),
        }
        if reset_vars.trim() == "y" {
            env_vars::reset_env_vars();
            run().await;
        } else {
            run().await;
        }
    } else {
        env_vars::set_env_vars();
        run().await;
    }
}

async fn run() {
    let intro_text = startup::introduction();
    println!("{intro_text}");

    // handle potential errors - status codes
    program_loop::program_loop().await;
}
