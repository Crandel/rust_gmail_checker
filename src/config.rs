use std::fs::File;
use std::io::Read;
use std::env;

pub fn get_config(config_file: &str) -> String {
    let mut json_path: String = String::from("");
    match env::home_dir() {
        Some(path_obj) => {
            match path_obj.to_str() {
                Some(path) => json_path = String::from(path),
                None => println!("Wrong path"),
            }
        }
        None => println!("Impossible to get your home dir!"),
    }
    json_path = format!("{}/{}", json_path, config_file);
    println!("{}", json_path);
    let mut file = match File::open(&json_path) {
        Err(why) => panic!("couldn't open {}: {}", json_path,
                                                   why),
        Ok(file) => file,
    };

    let mut data = String::new();
    if let Err(why) = file.read_to_string(&mut data) {
         panic!("couldn't read {}: {}", json_path, why);
    }
    return data;
}
