use std::{fs::File, path::PathBuf};
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write, ErrorKind};


pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file_path);
    }
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let filename = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ErrorKind::InvalidData)?
        .as_nanos()
        .to_string();

        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(filename);

        let file = File::create(&file_path)?;

        return Ok(Self {
            file_path: file_path,
            file: file,
        });
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let write = OpenOptions::new().write(true).open(&self.file_path)?;
        let mut write = BufWriter::new(write);

        write.write_all(&data)?;
        Ok(())

    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}
