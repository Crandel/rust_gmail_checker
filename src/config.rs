use std::fs::File;
use std::io::{Read,ErrorKind};
use std::env;
use accounts::Account;

pub fn get_config(config_file: &str) -> Vec<Account> {
    let mut json_path: String = String::from("");
    if let Some(path_obj) = env::home_dir() {
        if let Some(path) = path_obj.to_str() {
            json_path = String::from(path);
        }
    }
    json_path = format!("{}/{}", json_path, config_file);
    println!("{}", json_path);
    // let mut file = match File::open(&json_path) {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("json_path") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //             Err(error) => {
    //                 panic!("couldn't open {}: {}", json_path,
    //                        error);
    //             },
    //         };
    //     }
    // };
    let f = File::open(&json_path);
    let mut file = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(&json_path) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    let mut data = String::new();
    if let Err(why) = file.read_to_string(&mut data) {
        panic!("couldn't read {}: {}", json_path, why);
    }
    let acc = Account::new(
        String::from("username"),
        String::from("Short"),
        String::from("email"),
        String::from("password"),
    );
    let acc_vec = vec![acc];
    acc_vec
}
