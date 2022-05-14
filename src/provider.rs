use crate::{accounts::Account, client::WebClientError};
use async_trait::async_trait;
use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;

#[async_trait]
pub trait MailProvider {
    async fn get_mail_metadata(
        &self,
        acc: &Account,
        client: &Client<HttpsConnector<HttpConnector>, Body>,
    ) -> Result<String, WebClientError>;
    fn parse_body(body: String) -> Result<String, WebClientError>;
}
