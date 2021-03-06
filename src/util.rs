use std::path::PathBuf;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
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

    pub fn title(&self) -> String {
        self.path.file_stem().unwrap()
            .to_str().unwrap().to_owned()
    }

    pub fn timestamp(&self) -> std::io::Result<u64>  {
        let last_modified = self.path.metadata()?
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

        Ok(last_modified)
    }

}
