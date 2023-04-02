use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub creation_time: SystemTime,
}

impl FileInfo {
    pub fn new(path: PathBuf, creation_time: SystemTime) -> Self {
        Self {
            path,
            creation_time,
        }
    }
}


impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = self.creation_time.duration_since(UNIX_EPOCH).unwrap();
        let seconds = duration.as_secs();
        let creation_date = chrono::NaiveDateTime::from_timestamp(seconds as i64, 0);
        write!(f, "{} - {}", creation_date.format("%Y-%m-%d %H:%M:%S"), self.path.display())
    }
}

