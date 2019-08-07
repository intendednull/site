use std::path::PathBuf;
use chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct File {
    pub path: PathBuf
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap()
            .to_str().unwrap().to_owned()
    }

    pub fn timestamp(&self) -> std::io::Result<DateTime<Utc>>  {
        let last_modified = self.path.metadata()?
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs() as i64;

        Ok(Utc.timestamp(last_modified, 0))
    }

}
