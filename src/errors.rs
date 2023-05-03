use thiserror::Error;

pub fn handle_error(message: &str) {
    eprintln!("Error: {}", message);
}

#[derive(Error, Debug)]
pub enum EssenError {
    #[error("Web3 Error: {0}")]
    Web3(String, #[source] web3::Error),

    #[error("Argument error: {0}")]
    Arg(String),

    #[error("Parse Int error: {0}")]
    ParseInt(String, #[source] std::num::ParseIntError),

    #[error("error: {0}")]
    Block(String),

    #[error("csv error: {0}")]
    Csv(String, #[source] csv::Error),

    #[error("std error: {0}")]
    Std(String, #[source] std::io::Error),

    #[error("Failed to find configuration directory")]
    ConfigDirNotFound,

    #[error("{0}")]
    IOError(String, #[source] std::io::Error),

    #[error("{0}")]
    JsonError(String, #[source] serde_json::Error),

    #[error("Failed to read the stored config data")]
    ConfigReadError,

    #[error("Failed to parse the stored config data")]
    ConfigParseError,

    #[error("Failed to get infura project id")]
    InfuraProjectIdError,
}
