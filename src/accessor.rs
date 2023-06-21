pub trait TokenAccessor {
    fn get_token(&self, client_id: String, client_secret: String)
        -> Result<String, WebClientError>;
}
