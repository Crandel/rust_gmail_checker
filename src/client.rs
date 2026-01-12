use thiserror::Error;

/// Custom errors that may happen during calls
#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Client error: {:?}", _0)]
    ClientError(hyper::Error),
    #[error("Parsing error: {:?}", _0)]
    ParsingError(String),
    #[error("Connection error: {:?}", _0)]
    ConnectionError(String),
    #[error("Token error: {:?}", _0)]
    TokenError(String),
}
