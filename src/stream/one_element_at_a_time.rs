use std::fs::File;
use std::marker::PhantomData;
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

pub struct SimpleOutputStream<T: Display> {
    file_handle: File,
    resource_type: PhantomData<T>,
}

impl<T: Display> SimpleOutputStream<T> {
    pub fn create(file_path: impl Into<PathBuf>) -> SimpleOutputStream<T> {
        let file_handle = File::create(file_path.into()).unwrap();
        SimpleOutputStream {
            file_handle,
            resource_type: PhantomData,
        }
    }
}

impl<T: Display> OutputStream for SimpleOutputStream<T> {
    type Item = T;
    fn write(&mut self, element: Self::Item) -> () {
        let string_repr: String = format!("{}", element);
        let escape_line = string_repr + "\n";
        self.file_handle.write(escape_line.as_bytes()).unwrap();
    }

    fn close(&mut self) -> () {
        self.file_handle.flush();
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
        let mut stream = SimpleOutputStream::create(output_file_path.clone());
        stream.write(1);
        assert_eq!(output_file_path.exists(), true);
    }

    #[test]
    fn write_arbitrary_data_to_stream() {
        let tmp_dir = TempDir::new().unwrap();
        let buf = tmp_dir.path().join("opFile.txt");
        let mut stream = SimpleOutputStream::create(buf.as_path());
        stream.write("Hello");
        stream.close();
    }
}
