#![allow(dead_code)]  

mod client;
mod helpers;
mod requests;
mod run;

use helpers::env_vars;

use crate::helpers::startup;
use crate::run::program_loop;

#[tokio::main]
async fn main() {
    // lets check if our env_vars are set if true program starts running
    // otherwise let's ask user to set them and try again.
    let mut env_vars = helpers::env_vars::check_env_var_exists();
    loop {
        if env_vars {
            let intro_text = startup::introduction();
            println!("{intro_text}");

            // handle potential errors - status codes
            program_loop::program_loop().await;
        } else {
            env_vars::set_env_vars();
            env_vars = true;
            continue;
        }
    }
}
