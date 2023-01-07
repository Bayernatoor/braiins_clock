extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub fn set_env_vars(var: &str) -> String {  
    dotenv().ok(); 
    let mut key_to_return: String  = String::new(); 
    for (key, value) in env::vars() {
        if key == var {
           key_to_return = value; 
        }; 
    };
    return key_to_return;
}

pub fn check_env_var_exists() -> bool {
    dotenv().ok(); 
    let keys =  ["SLUSHPOOL_API_KEY", "BLOCKCLOCK_IP"];

    for key in &keys {
        if let Ok(val) = env::var(key) {
            println!("Key: {}, is set", key);
        } else {
            println!("Key: {}, is NOT set", key);
            println!("Please make sure to set them in .env file");
            return false
        };
    };
    return true 
}


