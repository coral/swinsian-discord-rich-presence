use crate::error::SwinsianError;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::{ops::Add, process::Command, time::Duration};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub swinsian: Option<Swinsian>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Swinsian {
    pub format: String,
    pub state: State,
    pub song: String,
    pub artist: String,
    pub album: String,
    pub pos: String,
    pub dur: String,
}

impl Swinsian {
    #[allow(non_snake_case)]
    pub fn calculate_POGRESS(&self) -> Option<i64> {
        let position: f32 = self.pos.parse().ok()?;
        let duration: f32 = self.dur.parse().ok()?;

        let diff = Duration::from_secs((duration - position) as u64);

        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()?
            .add(diff)
            .as_secs() as i64;

        Some(t)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Playing,
    Paused,
    Stopped,
    Unknown,
}

impl Default for State {
    fn default() -> Self {
        Self::Unknown
    }
}

pub fn get() -> Result<Swinsian, SwinsianError> {
    let r = Command::new("osascript").arg("swinsian.scpt").output()?;

    if r.stdout.is_empty() {
        let s = String::from_utf8_lossy(&r.stderr);
        return Err(SwinsianError::OsascriptOutputEmpty(format!("{}", s)));
    }

    let p: Request = serde_json::from_slice(&r.stdout)?;

    match p.swinsian {
        Some(v) => Ok(v),
        None => Err(SwinsianError::NoData),
    }
}
