// Base Error type for the library
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    APIError(String),
    ClientError(String),
    MaxRetriesExceeded(String)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}