extern crate gmail_lib;
extern crate hyper;
extern crate itertools;
extern crate tokio;

use hyper::Uri;
use itertools::Itertools;
use tokio::runtime::Runtime;

use gmail_lib::client::WebClient;
use gmail_lib::config;
use gmail_lib::gmail::GmailHandler;
use gmail_lib::utils::{Basic, ServiceUrl};

fn main() {
    let mut account_messages = Vec::new();
    // config filename
    let config_file = ".email.json";
    // gmail url
    // get data from config file
    let data = config::get_config_data(config_file);
    // extract accs info from Result
    let accs = match data {
        Ok(accs) => accs,
        Err(e) => {
            let error = match e {
                config::ConfigError::FileError(s) => s,
                _ => String::from(""),
            };
            println!("{}", error);
            return;
        }
    };
    let web_client = WebClient::new();

    let mut runtime = Runtime::new().unwrap();
    let gmail_handler = GmailHandler::new();

    // get number of unreaded messages for each acc
    for acc in &accs {
        let basic = Basic::new(
            String::from(acc.get_email()),
            String::from(acc.get_password()),
        );
        let base_str = basic.encode_tostr();
        let uri: Uri = gmail_handler.get_url().parse().unwrap();
        let response = runtime.block_on(web_client.send(uri, &base_str)).unwrap();
        // extract necessary info using Regex
        let result = gmail_handler.extract_result(response);
        // Save result as String
        account_messages.push(format!("{}:{}", acc.get_short(), result));
    }

    println!("{}", account_messages.iter().rev().join(" "));
}
