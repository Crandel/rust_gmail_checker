use crate::utils::{encode_tostr, Extractor};
use async_trait::async_trait;
use failure::Fail;
use hyper::{
    client::HttpConnector,
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, Uri,
};
use hyper_tls::HttpsConnector;

/// Custom errors that may happen during calls
#[derive(Debug, Fail)]
pub enum WebClientError {
    #[fail(display = "Hyper error: {:?}", _0)]
    HyperError(hyper::Error),
    #[fail(display = "Connection error: {:?}", _0)]
    ConnectionError(String),
}

#[async_trait]
pub trait WebClient {
    async fn send(&self, username: &str, password: &str, handler: &dyn Extractor) -> String;
}

pub struct WebClientImpl {
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl Default for WebClientImpl {
    fn default() -> Self {
        let https = HttpsConnector::new();
        let client: Client<_, Body> = Client::builder().build::<_, hyper::Body>(https);
        Self { client }
    }
}

impl WebClientImpl {
    async fn get_request(&self, username: &str, password: &str, url: String) -> Request<Body> {
        let uri = Uri::from_static(&url.to_owned());

        let auth_str = encode_tostr(username, password);

        // Await the response...
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        request
            .headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_str(&auth_str).unwrap());
        request
    }
}

#[async_trait]
impl WebClient for WebClientImpl {
    async fn send(&self, username: &str, password: &str, handler: &dyn Extractor) -> String {
        let url = handler.get_url();
        let request = self
            .get_request(username, password, String::from(url))
            .await;
        let resp = self.client.request(request).await.unwrap();
        let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&bytes)
            .expect("Valid utf-8 string")
            .to_string();
        let body = handler.extract_result(body_str);
        String::from(format!("{}:{}", "Z", body))
    }
}
