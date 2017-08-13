extern crate serde_json;

use std::fs::File;
use std::io::{Read,ErrorKind, Error};
use std::env;
use accounts::Account;

pub enum ConfigError{
    FileError(String),
    IOError(Error),
}

pub fn get_config(config_file: &str) -> Result<Vec<Account>, ConfigError> {
    // Get home directory
    let mut json_path: String = String::from("");
    match env::home_dir() {
        Some(path_obj) => {
            match path_obj.to_str() {
                Some(path) => json_path = format!("{}/{}", path, config_file),
                None => return Err(ConfigError::FileError(String::from("Impossible to get your home dir!"))),
            }
        },
        None => return Err(ConfigError::FileError(String::from("Impossible to get your home dir!"))),
    }

    let f = File::open(&json_path);
    let mut file = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(&json_path) {
                Ok(fc) => {
                    let acc = Account::new(
                        String::from("username"),
                        String::from("Short"),
                        String::from("email"),
                        String::from("password"),
                    );
                    let def_vec_acc = vec![acc];
                    let ser = serde_json::to_string(&acc).unwrap();
                    fc.write_all(ser);
                    fc
                },
                Err(e) => return Err(ConfigError::IOError(e)),
            }
        },
        Err(error) => return Err(ConfigError::IOError(error)),
    };
    let mut data = String::new();
    file.read_to_string(&mut data).expect(&format!("couldn't read to string {}", &json_path));
    println!("{}", data);
    let acc_vec: Vec<Account> = serde_json::from_str(&data).unwrap();
    Ok(acc_vec)
}
