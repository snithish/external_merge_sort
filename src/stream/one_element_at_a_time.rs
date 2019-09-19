use std::fs::File;
use std::path::PathBuf;

use crate::stream::io_streams::{InputStream, OutputStream};
use std::fmt::Display;
use std::io::Write;

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

    fn write<T: Display>(&self, element: T) -> () {
        let string_repr: String = format!("{}", element);
        let escape_line = string_repr + "\n";
        let mut file = self.file_handle.as_ref().unwrap();
        file.write(escape_line.as_bytes()).unwrap();
    }

    fn close(&mut self) -> () {
        self.file_handle.as_ref().unwrap().flush();
        self.file_handle = None;
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

    #[test]
    fn write_arbitrary_data_to_stream() {
        let tmp_dir = TempDir::new().unwrap();
        let buf = tmp_dir.path().join("opFile.txt");
        let mut stream = SimpleOutputStream::new();
        stream.create("generated_files/one.txt").unwrap();
        stream.write("Hello");
        stream.close();
    }
}
