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
}
