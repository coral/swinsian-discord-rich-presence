use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwinsianError {
    #[error("osascript error")]
    OsascriptError(#[from] std::io::Error),
    #[error("Applescript output empty, stderr: `{0}`")]
    OsascriptOutputEmpty(String),
    #[error("swinsian parse error")]
    SwinsianParseError(#[from] serde_json::Error),
    #[error("no data in request")]
    NoData,
    #[error("no data in request")]
    IPCError(#[from] Box<dyn std::error::Error>),
}
