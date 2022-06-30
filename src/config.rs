use crate::accounts::{Account, EmailType};
use dirs;
use serde_json;
use std::fs::File;
use std::io::{Error, Read, Write};

#[derive(Debug)]
pub enum ConfigError {
    FileError(String),
    IOError(Error),
}

const CONFIG_FILE: &str = ".email.json";

pub fn create_example() -> Result<(), ConfigError> {
    let json_path: String = get_config_path();
    let acc = Account::new(
        EmailType::Gmail,
        String::from("username"),
        String::from("Short"),
        String::from("email"),
        String::from("password"),
    );
    let def_vec_acc = vec![acc];
    let ex_acc_s: String = match serde_json::to_string(&def_vec_acc) {
        Ok(vec_str) => vec_str,
        Err(e) => return Err(ConfigError::FileError(e.to_string())),
    };
    let mut f: File = match File::create(&json_path) {
        Ok(f) => f,
        Err(e) => return Err(ConfigError::FileError(e.to_string())),
    };
    if let Err(e) = f.write_all(ex_acc_s.as_bytes()) {
        return Err(ConfigError::FileError(e.to_string()));
    };
    Ok(())
}

fn get_config_path() -> String {
    // Get home directory
    let path = dirs::home_dir().unwrap();
    let path_obj = path.to_str().unwrap();
    format!("{}/{}", path_obj, CONFIG_FILE)
}

pub fn get_config_data() -> Result<Vec<Account>, ConfigError> {
    let json_path: String = get_config_path();

    let mut file: File = match File::open(&json_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(ConfigError::FileError(format!(
                "File {} not found. Use --init to create a sample config",
                &json_path
            )))
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let accs: Result<Vec<Account>, ConfigError> = serde_json::from_str(&data).map_err(|err| {
        ConfigError::FileError(format!("Extracting from json failed with error {}", err))
    });
    accs
}
