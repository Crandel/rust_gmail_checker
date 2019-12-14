mod accounts;
mod client;
mod config;
mod gmail;
mod utils;

use client::WebClient;
use config;
use gmail::GmailHandler;
use utils::{EmailType, ServiceUrl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let gmail_handler: GmailHandler = Default::default();

    // get number of unreaded messages for each acc
    for acc in &accs {
        let handler = match acc.get_mail_type() {
            EmailType::Gmail => &gmail_handler,
            _ => &gmail_handler,
        };

        web_client.send(handler.get_url(), acc.get_email(), acc.get_password());
        let mut result = String::from("E");
        // extract necessary info using Regex
        if response.is_ok() {
            result = handler.extract_result(response.unwrap());
        }
        // Save result as String
        account_messages.push(format!("{}:{}", acc.get_short(), result));
    }

    println!("{}", account_messages.iter().rev().join(" "));
}
