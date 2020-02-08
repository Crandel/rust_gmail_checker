use std::time::Duration;

use failure::Fail;
use futures::Future;
use futures::Stream;
use hyper::{
    client::{
        HttpConnector,
        ResponseFuture
    },
    header::{
        HeaderMap,
        HeaderValue,
        AUTHORIZATION
    },
    Body,
    Client,
    Method,
    Request,
    Uri
};
use hyper_tls::HttpsConnector;

use crate::utils::Basic;

/// Custom errors that may happen during calls
#[derive(Debug, Fail)]
pub enum WebClientError {
    #[fail(display = "Hyper error: {:?}", _0)]
    HyperError(hyper::Error),
    #[fail(display = "Connection error: {:?}", _0)]
    ConnectionError(String),
}

pub trait WebClient {
    fn send(
        &self,
        url: &str,
        username: &str,
        password: &str,
    ) -> ResponseFuture;
}

pub struct WebClientImpl {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl Default for WebClientImpl {
    fn default() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client: Client<_, Body> = Client::builder()
            .keep_alive_timeout(Some(Duration::from_secs(20)))
            .build(https);
        Self { client }
    }
}

impl WebClient for WebClientImpl {
    fn send(
        &self,
        url: &str,
        username: &str,
        password: &str,
    ) -> ResponseFuture {
        let uri: Uri = url.parse::<Uri>().unwrap();
        let headers = self.create_headers(username, password);

        let basic = Basic::new(String::from(username), String::from(password));
        let base_str = basic.encode_tostr();
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())
            .unwrap();
        request.headers_mut().insert(AUTHORIZATION, HeaderValue::from_str(base_str.as_str()).unwrap());
        self.client.request(request)
    }
}
