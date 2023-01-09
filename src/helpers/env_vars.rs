extern crate dotenv;

use dotenv::dotenv;
use std::{env, io::Write, fs, io};

pub fn load_env_vars(var: &str) -> String {  
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
        if let Ok(_val) = env::var(key) {
            println!("Key: {}, is set", key);
        } else {
            println!("{}, is NOT set", key);
            set_env_vars();
        };
    };
    return true 
}

pub fn set_env_vars() -> () {
    let keys =  ["SLUSHPOOL_API_KEY", "BLOCKCLOCK_IP"];
    for key in &keys{
        println!("Please enter the value for {}: ", key);
        let mut result = String::new();
        let path = "./.env";
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read line");

        let value = format!("{}={}\n", key, result);
        let value_to_bytes = value.into_bytes();

        let mut file = match fs::OpenOptions::new().append(true).create(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("There was an error opening or creating the file: {}", e);
                return;
            },
        };

        if let Err(e) = file.write_all(&value_to_bytes) {
            println!("There was an error writing to the file {}", e);
        } else {
            println!("Successfully wrote to file");
        }
    };
}
