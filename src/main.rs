use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use mail_lib::{
    accounts::Account, client::WebClientError, config, provider::MailProvider,
    providers::gmail::GmailProvider,
};
use std::env::args;
use std::process::exit;

async fn process_accs<T: MailProvider>(
    accs: Vec<Account>,
    client: &Client<HttpsConnector<HttpConnector>, Body>,
    provider: T,
) -> Vec<Result<String, WebClientError>> {
    accs.iter()
        .map(|a| provider.get_mail_metadata(a, client))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
}

fn print_help() {
    println!(
        "-h - help message\n
--help - help message\n
--init - create sample config file\n
"
    );
    exit(0)
}

#[tokio::main]
async fn main() {
    // Prints each argument on a separate line
    for argument in args() {
        match argument.as_str() {
            "-h" => print_help(),
            "--help" => print_help(),
            "--init" => {
                if let Err(config::ConfigError::FileError(e)) = config::create_example() {
                    eprint!("File error{}", e)
                };
                return;
            }
            _ => continue,
        }
    }

    let data = config::get_config_data();
    // extract accs info from Result
    let accs = match data {
        Ok(accs) => accs,
        Err(e) => {
            match e {
                config::ConfigError::FileError(s) => eprint!("File error: {}", s),
                e => eprintln!("{:?}", e),
            };
            return;
        }
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let provider = GmailProvider::new();

    let resp_vec = process_accs(accs, &client, provider).await;
    let (responses, errors) =
        resp_vec
            .into_iter()
            .fold((Vec::new(), Vec::new()), |(mut strs, mut errs), current| {
                match current {
                    Ok(s) => strs.push(s),
                    Err(e) => errs.push(e),
                }
                (strs, errs)
            });
    for error in errors {
        eprintln!("{}", error);
    }
    println!("{}", responses.join(" "));
}
