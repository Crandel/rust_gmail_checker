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

pub struct WebClient {
    client: Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl Default for WebClient {
    fn default() -> Self {
        let https = HttpsConnector::new(4).unwrap();
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
    ) -> impl Future<Item = String, Error = WebClientError> {
        let uri: Uri = url.parse::<Uri>().unwrap();
        let headers = self.create_headers(username, password);

        self.send_internal(uri, &headers)
            .map_err(WebClientError::HyperError)
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
            })
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
