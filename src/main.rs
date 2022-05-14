use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use mail_lib::{
    accounts::Account, client::WebClientError, config, provider::MailProvider,
    providers::gmail::GmailProvider,
};

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

#[tokio::main]
async fn main() {
    // config filename
    let config_file = ".email.json";
    // gmail url
    // get data from config file
    let data = config::get_config_data(config_file);
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
