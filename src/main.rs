use gmail_lib;
use itertools::Itertools;
use tokio::runtime::Runtime;
use hyper::client::ResponseFuture;
use futures::{
    future::{
        join_all,
        JoinAll
    },
    Future,
    Stream
};

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
                            acc.get_password())
        }).collect();

    let join_futures: JoinAll<Vec<ResponseFuture>> = join_all(account_futures);
    let responses = runtime.block_on(join_futures);
    let account_strings: Vec<String> = responses.into_iter().map(
        |resp| {
            let response = resp.map_err(WebClientError::HyperError)
                .and_then(|response| {
                    let is_success = response.status().is_success();
                    response.into_body().concat2().then(move |result| {
                        let chunk = result.map_err(WebClientError::HyperError)?;
                        if is_success {
                            let bytes = chunk.into_bytes();
                            let text: String = String::from_utf8_lossy(&bytes).into_owned();
                            Ok(text)
                        } else {
                            let bytes = chunk.into_bytes();
                            let text: String = String::from_utf8_lossy(&bytes).into_owned();
                            Err(WebClientError::ConnectionError(text))
                        }
                    })
                });
            let body = match response {
                // extract necessary info using Regex
                Ok(bod) => gmail_handler.extract_result(bod),
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
            String::from(format!("{}:{}", "acc.get_short()", body))
        }).collect();

    println!("{}", account_strings.iter().join(" "));
}
