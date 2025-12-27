use crate::{accounts::Account, client::InternalError};
use async_trait::async_trait;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

#[async_trait]
pub trait MailProvider {
    async fn get_mail_metadata(
        &self,
        acc: &Account,
        client: &Client<HttpsConnector<HttpConnector>, Empty<Bytes>>,
    ) -> Result<String, InternalError>;
    fn parse_body(body: String) -> Result<String, InternalError>;
}
