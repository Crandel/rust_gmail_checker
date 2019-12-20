use std::time::Duration;

use failure::Fail;
use futures::Future;
use hyper::{
    client::{
        HttpConnector,
        ResponseFuture
    },
    Body,
    Client,
    Error,
    Method,
    Request,
    Response,
    Uri
};
use http::header::{
    HeaderMap,
    HeaderValue,
    AUTHORIZATION
};
use hyper_tls::HttpsConnector;

use crate::utils::Basic;

/// Custom errors that may happen during calls
#[derive(Debug, Fail)]
pub enum WebClientError {
    #[fail(display = "Hyper error: {:?}", _0)]
    HyperError(Error),
    #[fail(display = "Connection error: {:?}", _0)]
    ConnectionError(String),
}

pub struct WebClient {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl Default for WebClient {
    fn default() -> Self {
        let https = HttpsConnector::new();
        let client: Client<_, Body> = Client::builder()
            .keep_alive_timeout(Some(Duration::from_secs(20)))
            .build(https);
        Self { client }
    }
}

impl WebClient {
    pub fn send(
        &self,
        url: &str,
        username: &str,
        password: &str
    ) -> ResponseFuture {
        let uri: Uri = url.parse::<Uri>().unwrap();
        let headers = self.create_headers(username, password);

        self.send_internal(uri, &headers)
    }

    fn send_internal(&self, url: Uri, headers: &HeaderMap) -> ResponseFuture {
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())
            .unwrap();
        for (key, value) in headers {
            request.headers_mut().insert(key, value.clone());
        }
        self.client.request(request)
    }

    fn create_headers(&self, username: &str, password: &str) -> HeaderMap {
        let basic = Basic::new(
            String::from(username),
            String::from(password)
        );
        let base_str = basic.encode_tostr();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(base_str.as_str()).unwrap(),
        );
        headers
    }

}
