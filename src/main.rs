extern crate gmail_lib;

use gmail_lib::config;

fn main(){
    let config_file = ".gmail1.json";
    let data = config::get_config(config_file);
    let accs = match data {
        Ok(accs) => accs,
        Err(e) => {
            let error = match e {
                config::ConfigError::FileError(s) => s,
                _ => String::from(""),
            };
            println!("{}", error);
            return
        },
    };
    println!("{:?}", accs);
}
