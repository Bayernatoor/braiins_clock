extern crate dotenv;

use dotenv::dotenv;
use std::{env, fs, io, io::Write};

/*
var supplied as parameter is matched against exisiting env var and value is
returned to the caller if there's a match.
*/
pub fn load_env_vars(var: &str) -> String {
    dotenv().ok();
    let mut key_to_return: String = String::new();
    for (key, value) in env::vars() {
        if key == var {
            key_to_return = value;
        };
    }
    return key_to_return;
}

// runs on program start and ensures that env vars are set
pub fn check_env_var_exists() -> bool {
    dotenv().ok();
    let keys = ["SLUSHPOOL_API_KEY", "BLOCKCLOCK_IP"];
    let mut is_set = false;
    println!("Ensuring that environment variables are set.\n");
    for key in &keys {
        if let Ok(_val) = env::var(key) {
            println!("{key}, is set");
            is_set = true;
        } else {
            println!("{key} is NOT set");
            return false;
        };
    }
    return is_set;
}

// prompts user to enter values for environment variables.
pub fn set_env_vars() {
    let keys = ["SLUSHPOOL_API_KEY", "BLOCKCLOCK_IP"];
    for key in &keys {
        println!("Please enter the value for {key}: ");
        let mut result = String::new();
        let path = "./.env";
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read line");

        let value = format!("{key}={result}\n");
        let value_to_bytes = value.into_bytes();

        // appends value to exisiting .env or creates it if necessary
        let mut file = match fs::OpenOptions::new().append(true).create(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("There was an error opening or creating the file: {e}");
                return;
            }
        };

        // attempts to write value to file.
        if let Err(e) = file.write_all(&value_to_bytes) {
            println!("There was an error writing to the file {e}");
        } else {
            println!("Successfully wrote to file");
        }
    }
}

pub fn reset_env_vars() {
    let path = "./.env";
    match fs::File::create(path) {
        Ok(_file) => set_env_vars(),
        Err(e) => {
            println!("Error truncating .env file: {e}");
        }
    };
}
