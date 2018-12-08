use futures::Future;
use futures::Stream;
use hyper;
use hyper::client::HttpConnector;
use hyper::client::ResponseFuture;
use hyper::header::HeaderMap;
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

impl Default for WebClient {
    fn default() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client: Client<_, Body> = Client::builder().build(https);
        Self { client }
    }
}

impl WebClient {
    pub fn send(
        &self,
        url: Uri,
        header: &HeaderMap,
    ) -> impl Future<Item = String, Error = WebClientError> {
        self.send_internal(url, header)
            .map_err(|e| WebClientError::HyperError(e))
            .and_then(|response| {
                let is_success = response.status().is_success();
                response.into_body().concat2().then(move |result| {
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
}
