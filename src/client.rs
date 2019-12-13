use reqwest::{Client, Error, Response};
use futures::Future;

pub struct WebClient {
    client: Client,
}

impl Default for WebClient {
    fn default() -> Self {
        let clientBuilderResult = reqwest::Client::builder()
            .build();
        let client = if let Right(cl) = client {
                cl
        };
        Self { client }
    }
}

impl WebClient {
    pub fn send(
        &self,
        url: String,
        user: String,
        password: String
    ) -> impl Future<Output = Result<Response, Error>> {
        self.client
            .get(url)
            .basic_auth(user, Some(password))
            .send()
    }
}
