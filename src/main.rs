use base64;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use hyper::{
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use mail::{
    accounts::Account,
    config,
    client::{
        RequestInfo,
        WebClientImpl
    },
};

#[tokio::main]
fn main() {
    // get data from config file
    let data = config::get_config_data();
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

    let gmail_handler: GmailHandler = Default::default();

    // get number of unreaded messages for each acc
    let account_strings: Vec<String> = accs
        .into_iter()
        .map(|acc| {
            let handler = match acc.get_mail_type() {
                EmailType::Gmail => &gmail_handler,
                _ => &gmail_handler,
            };
            let request_info = RequestInfo::new(String::from(acc.get_email()), 
                                                String::from(acc.get_password()),
                                                handler)
            web_client.send(request_info)
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await

    println!("{}", account_strings.iter().join(" "));
}
