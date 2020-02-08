use gmail_lib;
use itertools::Itertools;
use tokio::runtime::Runtime;

use gmail_lib::{
    accounts::EmailType,
    client::{
        WebClientImpl,
        WebClientError
    },
    config,
    gmail::GmailHandler,
    utils::ServiceUrl
};
use hyper::client::ResponseFuture;

fn main() {
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
            eprintln!("{}", error);
            return;
        }
    };
    let web_client: WebClientImpl = Default::default();

    let mut runtime = Runtime::new().unwrap();
    let gmail_handler: GmailHandler = Default::default();

    // get number of unreaded messages for each acc
    let account_futures: Vec<ResponseFuture> = accs.into_iter().map(
        |acc| {
            let handler = match acc.get_mail_type() {
                EmailType::Gmail => &gmail_handler,
                _ => &gmail_handler,
            };

            web_client.send(handler.get_url(),
                            acc.get_email(),
                            acc.get_password()));
        }).collect()


            let response = runtime.block_on(
            let body = match response {
                Ok(bod) => handler.extract_result(bod),
                Err(e) => match e {
                    WebClientError::HyperError(he) => {
                        eprintln!("{}", he);
                        String::from("HE")
                    },
                    WebClientError::ConnectionError(ce) => {
                        eprintln!("{}", ce);
                        String::from("CE")
                    }
                }
            };
            // extract necessary info using Regex
            String::from(format!("{}:{}", acc.get_short(), body))
        }).collect();

    println!("{}", account_messages.iter().join(" "));
}
