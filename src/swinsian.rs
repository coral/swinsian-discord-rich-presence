use crate::error::SwinsianError;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub swinsian: Option<Swinsian>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Swinsian {
    pub format: String,
    pub state: String,
    pub song: String,
    pub artist: String,
    pub album: String,
}

pub fn get() -> Result<Swinsian, SwinsianError> {
    let r = Command::new("osascript").arg("as.scpt").output()?;

    let p: Request = serde_json::from_slice(&r.stdout)?;

    match p.swinsian {
        Some(v) => Ok(v),
        None => Err(SwinsianError::NoData),
    }
}
