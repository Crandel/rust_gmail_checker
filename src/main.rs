use gmail_lib;

use itertools::Itertools;

use hyper::{
    Client,
    body::HttpBody as _,
    Response
};
use tokio::io::{self, AsyncWriteExt as _};

use gmail_lib::{
    accounts::{Account, EmailType},
    client::WebClient,
    config,
    gmail::GmailHandler,
    utils::ServiceUrl
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() -> Result<()>{
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
    let account_messages: Vec<String> = accs.into_iter().map(|acc| {
        call_mail(acc, gmail_handler, web_client)
    }).collect();

    println!("{}", account_messages.iter().rev().join(" "));
    Ok(())
}

async fn call_mail(acc: Account, service: ServiceUrl, client: WebClient) -> Result<String> {
    let handler = match acc.get_mail_type() {
        EmailType::Gmail => &service,
        _ => &service,
    };

    let response = client.send(handler.get_url(),
                               acc.get_email(),
                               acc.get_password()).await;

    match response {
        Ok(Response(body)) => Ok(handler.extract_result(body)),
        Err(e)=> Err(e)
    }
}
