#![allow(dead_code)]

mod client;
mod helpers;
mod requests;
mod run;

use helpers::env_vars;

use crate::helpers::startup;
use crate::run::program_loop;

#[tokio::main]
async fn main() -> () {
    let env_vars = helpers::env_vars::check_env_var_exists();

    if env_vars {
        let intro_text = startup::introduction();
        println!("{}", intro_text);
        
        // handle potential errors - status codes 
        program_loop::program_loop().await;
    } else {
        env_vars::set_env_vars(); 
    }
}

