extern crate gmail_lib;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate regex;

use std::str;
use futures::{Future, Stream};
use hyper::{Client, Uri, Method, Request};
use hyper_tls::HttpsConnector;
use hyper::header::{Authorization, Basic};
use tokio_core::reactor::Core;
use regex::Regex;

use gmail_lib::config;

fn main(){
    // result should be printed to stdout;
    let mut result_str = "".to_owned();
    // config filename
    let config_file = ".gmail.json";
    // gmail url
    let uri = "https://mail.google.com/mail/feed/atom";
    let uri = uri.parse::<Uri>().unwrap();
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
            return
        },
    };
    // creating http request
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    // Regex part
    let fullcount = Regex::new("<fullcount>(.*?)</fullcount>").unwrap();
    let re = Regex::new("[0-9+]").unwrap();

    // get number of unreaded messages for each acc
    for acc in &accs {
        let mut req = Request::new(Method::Get, uri.clone());
        {
            // base64 header with sensitive data
            let mut headers = req.headers_mut();
            headers.set(Authorization(Basic {
                username: String::from(acc.get_email()),
                password: Some(String::from(acc.get_password()))
            }));
        }
        let gmail = client.request(req).and_then(|res| {
            res.body().concat2()
        });
        let result = core.run(gmail).unwrap();
        // get info from response
        let body_str = match str::from_utf8(&result) {
            Ok(body) => body,
            _ => "",
        };
        // extract necessary info using Regex
        let mat = fullcount.find(body_str).unwrap();
        let fullcount_str = &body_str[mat.start()..mat.end()];
        let result = re.find(fullcount_str).unwrap();
        // Save result as String
        result_str.push_str(&format!("{}:{} ", acc.get_short(), &fullcount_str[result.start()..result.end()]));
    }
    println!("{}", result_str);
}
