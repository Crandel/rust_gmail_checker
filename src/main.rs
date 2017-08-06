extern crate gmail_lib;

use gmail_lib::config;

fn main(){
    let config_file = ".gmail.json";
    let data = config::get_config(config_file);
    println!("{}", data);
}
