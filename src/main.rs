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
use gmail_lib::utils::{EmailType, ServiceUrl};

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
    let web_client: WebClient = Default::default();

    let mut runtime = Runtime::new().unwrap();
    let gmail_handler: GmailHandler = Default::default();

    // get number of unreaded messages for each acc
    for acc in &accs {
        let handler = match acc.get_mail_type() {
            EmailType::Gmail => &gmail_handler,
            _ => &gmail_handler,
        };

        let uri: Uri = handler.get_url().parse().unwrap();
        let headers = handler.create_headers(acc);

        let response = runtime.block_on(web_client.send(uri, &headers)).unwrap();
        // extract necessary info using Regex
        let result = handler.extract_result(response);
        // Save result as String
        account_messages.push(format!("{}:{}", acc.get_short(), result));
    }

    println!("{}", account_messages.iter().rev().join(" "));
}
