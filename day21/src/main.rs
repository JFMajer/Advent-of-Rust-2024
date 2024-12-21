use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{fs::File, path::PathBuf};
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, Write};


pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

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
