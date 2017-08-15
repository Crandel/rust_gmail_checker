extern crate gmail_lib;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Uri, Method, Request};
use hyper::header::{Headers, Authorization, Basic};
use tokio_core::reactor::Core;

use gmail_lib::config;

fn main(){
    let config_file = ".gmail.json";
    let mut uri = "https://mail.google.com/mail/feed/atom";
    let mut uri = uri.parse::<hyper::Uri>().unwrap();
    let data = config::get_config(config_file);
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
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    for acc in &accs {
        let mut req = Request::new(Method::Get, uri.clone());
        {
            let mut headers = req.headers_mut();
            headers.set(Authorization(Basic {
                username: String::from(acc.get_email()),
                password: Some(String::from(acc.get_password()))
            }));
        }
        let mut result = client.request(req);
        println!("{:?}", result);
    }
}
