use crate::client::InternalError;

pub trait TokenAccessor {
    /// Return token from the source.
    ///
    /// # Errors
    ///
    /// This function will return an error if it is not possible to get the token.
    fn get_token(&self, client_id: String, client_secret: String) -> Result<String, InternalError>;
}
