use crate::{
    accounts::Account,
    client::WebClientError,
    client::WebClientError::{ConnectionError, ParsingError},
    provider::MailProvider,
};
pub(crate) use async_trait::async_trait;
use hyper::{
    client::HttpConnector,
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request,
};
use hyper_tls::HttpsConnector;

use base64;
use roxmltree::Document;

pub struct GmailProvider {
    url: String,
}
impl Default for GmailProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl GmailProvider {
    pub fn new() -> GmailProvider {
        let url = String::from("https://mail.google.com/mail/feed/atom");
        GmailProvider { url }
    }

    fn get_request(&self, acc: &Account) -> Result<Request<Body>, WebClientError> {
        let user_data: String = format!("{}:{}", acc.get_email(), acc.get_password());
        let b64: String = base64::encode(user_data.as_bytes());
        let auth_str: String = format!("Basic {}", b64);

        let value: HeaderValue = HeaderValue::from_str(&auth_str).unwrap();
        // Await the response...
        Request::builder()
            .method(Method::GET)
            .uri(self.url.to_string())
            .header(AUTHORIZATION, value)
            .body(Body::empty())
            .map_err(|e| ConnectionError(e.to_string()))
    }
}

#[async_trait]
impl MailProvider for GmailProvider {
    async fn get_mail_metadata(
        &self,
        acc: &Account,
        client: &Client<HttpsConnector<HttpConnector>, Body>,
    ) -> Result<String, WebClientError> {
        // Parse an `http::Uri`...
        let request = self.get_request(acc);

        let resp = match request {
            Ok(r) => client
                .request(r)
                .await
                .map_err(|err| ConnectionError(err.to_string())),
            Err(e) => Err(e),
        };
        let bytes_res: Result<hyper::body::Bytes, WebClientError> = match resp {
            Ok(rsp) => hyper::body::to_bytes(rsp.into_body())
                .await
                .map_err(|er| ConnectionError(er.to_string())),
            Err(e) => Err(e),
        };
        let body_res: Result<String, WebClientError> = match bytes_res {
            Ok(bytes) => std::str::from_utf8(&bytes)
                .map(|by| by.to_string())
                .map_err(|er| ParsingError(er.to_string())),
            Err(e) => Err(e),
        };
        body_res.and_then(|body| {
            GmailProvider::parse_body(body).map(|count| format!("{}:{}", acc.get_short(), count))
        })
    }

    fn parse_body(body: String) -> Result<String, WebClientError> {
        match Document::parse(body.as_str()) {
            Ok(doc) => match doc.descendants().find(|n| n.has_tag_name("fullcount")) {
                Some(fc) => match fc.text() {
                    Some(count) => Ok(count.to_string()),
                    None => Err(ParsingError("Text in fullcount is missing".to_string())),
                },
                None => Err(ParsingError("fullcount not found".to_string())),
            },
            Err(er) => Err(ParsingError(er.to_string())),
        }
    }
}
