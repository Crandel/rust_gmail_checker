use std::fs::File;
use std::io::{Read,ErrorKind, Error, Write};
use std::env;
use accounts::Account;
use serde_json;

#[derive(Debug)]
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
            let mut sample_file = match File::create(&json_path) {
                Ok(fc) => fc,
                Err(e) => return Err(ConfigError::IOError(e)),
            };
            let acc = Account::new(
                        String::from("username"),
                        String::from("Short"),
                        String::from("email"),
                        String::from("password"),
            );
            let def_vec_acc = vec![acc];
            let ser = serde_json::to_string(&def_vec_acc).unwrap();
            match sample_file.write_all(ser.as_bytes()){
                Ok(fs) => return Err(ConfigError::FileError(String::from("There are no config file
Sample config file  '~/.gmail.json' was created, please fill all neccessary fields"))),
                Err(error) => return Err(ConfigError::IOError(error)),
            };
        },
        Err(error) => return Err(ConfigError::IOError(error)),
    };
    let mut data = String::new();
    file.read_to_string(&mut data).expect(&format!("couldn't read to string {}", &json_path));
    println!("{}", data);
    let acc_vec: Vec<Account> = serde_json::from_str(&data).unwrap();
    Ok(acc_vec)
}
