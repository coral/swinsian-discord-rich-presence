use crate::error::SwinsianError;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::SystemTime;
use std::{ops::Sub, time::Duration};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub swinsian: Option<SwinsianResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwinsianResponse {
    pub format: String,
    pub state: State,
    pub song: String,
    pub artist: String,
    pub album: String,
    pub pos: String,
    pub dur: String,
}

impl SwinsianResponse {
    #[allow(non_snake_case)]
    pub fn calculate_POGRESS(&self) -> Option<i64> {
        let position: f32 = self.pos.parse().ok()?;

        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()?
            .sub(Duration::from_secs(position as u64))
            .as_secs() as i64;

        Some(t)
    }

    pub fn artist(&self) -> String {
        if self.artist == "missing value" {
            return "".to_string();
        }
        self.artist.clone()
    }

    pub fn album(&self) -> String {
        if self.album == "missing value" {
            return "".to_string();
        }
        self.album.clone()
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

pub struct Swinsian {
    path: String,
}

impl Swinsian {
    pub fn new() -> Result<Swinsian, SwinsianError> {
        let script = include_bytes!("../swinsian-apple-script.scpt");
        let compile_path = "/tmp/compiled-swinsian-apple-script.scpt".to_string();

        let mut osacompile = Command::new("osacompile")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("-o")
            .arg(&compile_path)
            .spawn()?;

        let osacompile_stdin = osacompile.stdin.as_mut().unwrap();
        osacompile_stdin.write_all(script)?;
        drop(osacompile_stdin);

        let output = osacompile.wait_with_output()?;

        if !output.stderr.is_empty() {
            let s = String::from_utf8_lossy(&output.stderr);
            return Err(SwinsianError::OsascriptOutputEmpty(format!("{}", s)));
        }

        Ok(Swinsian { path: compile_path })
    }

    pub fn get(&self) -> Result<SwinsianResponse, SwinsianError> {
        let r = Command::new("osascript").arg(&self.path).output()?;

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
}
