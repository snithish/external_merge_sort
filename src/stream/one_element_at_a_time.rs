use std::fs::File;
use std::path::PathBuf;

use crate::stream::io_streams::{InputStream, OutputStream};

pub struct SimpleInputStream<T: 'static> {
    file: File,
    val: Vec<T>,
}

impl<T> InputStream<T> for SimpleInputStream<T> {
    fn open(&self, file_path: impl Into<PathBuf>) -> std::io::Result<()> {
        Ok(())
    }

    fn read_next(&self) -> T {
        unimplemented!()
    }

    fn end_of_stream(&self) -> bool {
        unimplemented!()
    }
}

pub struct SimpleOutputStream {
    file_handle: Option<File>,
}

impl SimpleOutputStream {
    fn new() -> SimpleOutputStream {
        SimpleOutputStream { file_handle: None }
    }
}
impl OutputStream for SimpleOutputStream {
    fn create(&mut self, file_path: impl Into<PathBuf>) -> std::io::Result<()> {
        self.file_handle = Some(File::create(file_path.into())?);
        Ok(())
    }

    fn write<T>(&self, element: T) -> () {
        unimplemented!()
    }

    fn close(&self) -> () {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn create_must_create_a_file_if_not_present() {
        let tmp_dir = TempDir::new().unwrap();
        let buf = tmp_dir.path().join("opFile.txt");
        let output_file_path = buf.as_path().clone();
        assert_eq!(output_file_path.exists(), false);
        SimpleOutputStream::new()
            .create(output_file_path.clone())
            .unwrap();
        assert_eq!(output_file_path.exists(), true);
    }
}
