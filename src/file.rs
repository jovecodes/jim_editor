use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Default)]
pub struct JimFile {
    pub path: PathBuf,
    pub contents: String,
}

impl JimFile {
    pub fn new(path: &Path) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            contents,
        })
    }

    pub fn save(&self) -> io::Result<()> {
        fs::write(&self.path, &self.contents)?;
        Ok(())
    }
}
