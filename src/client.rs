use thiserror::Error;

/// Custom errors that may happen during calls
#[derive(Error, Debug)]
pub enum WebClientError {
    #[error("Hyper error: {:?}", _0)]
    HyperError(hyper::Error),
    #[error("Parsing error: {:?}", _0)]
    ParsingError(String),
    #[error("Connection error: {:?}", _0)]
    ConnectionError(String),
    #[error("Token error: {:?}", _0)]
    TokenError(String),
}
