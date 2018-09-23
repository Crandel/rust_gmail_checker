use futures::Future;
use futures::Stream;
use hyper;
use hyper::client::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::header::{HeaderValue, AUTHORIZATION};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

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

impl WebClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client: Client<_, Body> = Client::builder().build(https);
        Self { client }
    }

    pub fn send(
        &self,
        url: Uri,
        header: &str,
    ) -> impl Future<Item = String, Error = WebClientError> {
        let result = self
            .send_internal(url, header)
            .map_err(|e| WebClientError::HyperError(e))
            .and_then(|response| {
                let is_success = response.status().is_success();
                let result = response.into_body().concat2().then(move |result| {
                    let chunk = result.map_err(|e| WebClientError::HyperError(e))?;
                    if is_success {
                        let bytes = chunk.into_bytes();
                        let text: String = String::from_utf8_lossy(&bytes).into_owned();
                        Ok(text)
                    } else {
                        let bytes = chunk.into_bytes();
                        let text: String = String::from_utf8_lossy(&bytes).into_owned();
                        Err(WebClientError::ConnectionError(text))
                    }
                });
                result
            });
        result
    }

    fn send_internal(&self, url: Uri, header: &str) -> ResponseFuture {
        let request = Request::builder()
            .method(Method::GET)
            .header(AUTHORIZATION, HeaderValue::from_str(header).unwrap())
            .uri(url)
            .body(Body::empty())
            .unwrap();
        self.client.request(request)
    }
}
