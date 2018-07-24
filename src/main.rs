extern crate futures;
extern crate gmail_lib;
extern crate hyper;
extern crate hyper_tls;
extern crate itertools;
extern crate regex;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::header::{Authorization, Basic};
use hyper::{Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use itertools::Itertools;
use regex::Regex;
use std::str;
use tokio_core::reactor::Core;

use gmail_lib::config;

fn main() {
    // save a
    let mut account_messages = Vec::new();
    // if at least one account has unread message, change color
    let mut has_unread = false;
    // config filename
    let config_file = ".gmail.json";
    let null_color = "#2E64FE";
    let unread_color = "#D0FA58";
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
            return;
        }
    };
    // creating http request
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    // Regex part
    let fullcount = Regex::new("<fullcount>(.*?)</fullcount>").unwrap();
    let count_number = Regex::new("[0-9+]").unwrap();

    // get number of unreaded messages for each acc
    for acc in &accs {
        let mut req = Request::new(Method::Get, uri.clone());
        {
            // base64 header with sensitive data
            let mut headers = req.headers_mut();
            headers.set(Authorization(Basic {
                username: String::from(acc.get_email()),
                password: Some(String::from(acc.get_password())),
            }));
        }
        let response = client.request(req).and_then(|res| res.body().concat2());
        let response_string = core.run(response).unwrap();
        // get info from response
        let body_str = match str::from_utf8(&response_string) {
            Ok(body) => body,
            _ => "",
        };

        // extract necessary info using Regex
        let result = match fullcount.find(body_str) {
            Some(count) => {
                let fullcount_str = &body_str[count.start()..count.end()];
                match count_number.find(fullcount_str) {
                    Some(res) => &fullcount_str[res.start()..res.end()],
                    None => "",
                }
            }
            None => "",
        };
        if result != "0" {
            has_unread = true;
        }

        // Save result as String
        account_messages.push(format!("{}:{}", acc.get_short(), result));
    }
    let result_color = if has_unread { unread_color } else { null_color };

    let count = format!(
        r#"{{ "full_text" : "\uF0E0 {}", "color" : "{}"}}"#,
        account_messages.iter().rev().join(" "),
        result_color
    );
    println!("{}", count);
}
