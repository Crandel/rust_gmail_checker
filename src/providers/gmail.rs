use crate::{
    accessor::TokenAccessor,
    accounts::Account,
    client::InternalError,
    client::InternalError::{ConnectionError, ParsingError},
    provider::MailProvider,
};
pub(crate) use async_trait::async_trait;
use hyper::{
    client::HttpConnector,
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request,
};
use hyper_tls::HttpsConnector;

use roxmltree::Document;

pub struct GmailProvider {
    feed_url: String,
    auth_url: String,
    token_url: String,
}
impl Default for GmailProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl GmailProvider {
    pub fn new() -> GmailProvider {
        let feed_url = String::from("https://mail.google.com/mail/feed/atom");
        let auth_url = String::from("https://mail.google.com/mail/feed/atom");
        let token_url = String::from("https://mail.google.com/mail/feed/atom");
        GmailProvider {
            feed_url,
            auth_url,
            token_url,
        }
    }

    fn get_request(&self, acc: &Account) -> Result<Request<Body>, InternalError> {
        let Some(client_secret) = acc.get_client_secret() else {
            return Err(InternalError::TokenError(String::from("secret err")));
        };
        let Ok(auth_token) = self.get_token(String::from(acc.get_client_id()), client_secret) else {
            return Err(InternalError::TokenError(String::from("token err")));
        };

        let value: HeaderValue = HeaderValue::from_str(&auth_token).unwrap();
        // Await the response...
        Request::builder()
            .method(Method::GET)
            .uri(self.feed_url.to_string())
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
    ) -> Result<String, InternalError> {
        // Parse an `http::Uri`...
        let request = self.get_request(acc);

        let resp = match request {
            Ok(r) => client
                .request(r)
                .await
                .map_err(|err| ConnectionError(err.to_string())),
            Err(e) => Err(e),
        };
        let bytes_res: Result<hyper::body::Bytes, InternalError> = match resp {
            Ok(rsp) => hyper::body::to_bytes(rsp.into_body())
                .await
                .map_err(|er| ConnectionError(er.to_string())),
            Err(e) => Err(e),
        };
        let body_res: Result<String, InternalError> = match bytes_res {
            Ok(bytes) => std::str::from_utf8(&bytes)
                .map(|by| by.to_string())
                .map_err(|er| ParsingError(er.to_string())),
            Err(e) => Err(e),
        };
        body_res.and_then(|body| {
            GmailProvider::parse_body(body).map(|count| format!("{}:{}", acc.get_short(), count))
        })
    }

    fn parse_body(body: String) -> Result<String, InternalError> {
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

impl TokenAccessor for GmailProvider {
    fn get_token(&self, client_id: String, client_secret: String) -> Result<String, InternalError> {
        let token = String::from("");
        return Ok(token);
    }
}
