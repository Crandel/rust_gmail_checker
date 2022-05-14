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

fn create_example() -> String {
    let acc = Account::new(
        EmailType::Gmail,
        String::from("username"),
        String::from("Short"),
        String::from("email"),
        String::from("password"),
    );
    let def_vec_acc = vec![acc];
    serde_json::to_string(&def_vec_acc).unwrap()
}

pub fn get_config_data(config_file: &str) -> Result<Vec<Account>, ConfigError> {
    // Get home directory
    let path = dirs::home_dir().unwrap();
    let path_obj = path.to_str().unwrap();
    let json_path: String = format!("{}/{}", path_obj, config_file);

    let mut file: File = match File::open(&json_path) {
        Ok(file) => file,
        Err(_) => {
            let mut f = File::create(&json_path).unwrap();
            let ex_acc_s = create_example();
            f.write_all(ex_acc_s.as_bytes()).unwrap();
            panic!("File {} not found. New one was created", &json_path)
        }
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let accs: Result<Vec<Account>, ConfigError> = serde_json::from_str(&data).map_err(|err| {
        ConfigError::FileError(format!("Extracting from json failed with error {}", err))
    });
    accs
}
