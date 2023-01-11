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
    let mut env_vars = helpers::env_vars::check_env_var_exists();
    println!("VARS in MAIN ARE: {}", env_vars);
    loop {
        if env_vars {
            println!("LETS GO!");
            let intro_text = startup::introduction();
            println!("{}", intro_text);
            
            // handle potential errors - status codes 
            program_loop::program_loop().await;
        } else {
            println!("Let's set those vars");
            env_vars::set_env_vars();
            env_vars = true;
            continue;
        }
    }
}

