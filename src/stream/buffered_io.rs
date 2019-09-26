use std::fs::File;
use std::path::PathBuf;

use crate::stream::io_streams::{InputStream, OutputStream};
use std::fmt::Display;
use std::io::Write;

pub struct BufferedStream<T: 'static> {
    file: File,
    val: Vec<T>,
}

impl<T> InputStream<T> for BufferedStream<T> {
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

pub struct BufferedOutputStream<T: Display> {
    file_handle: File,
    buffer: Vec<T>,
}

impl<T: Display> BufferedOutputStream<T> {
    pub fn create(file_path: impl Into<PathBuf>, capacity: usize) -> BufferedOutputStream<T> {
        let file_handle = File::create(file_path.into()).unwrap();
        BufferedOutputStream {
            file_handle,
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn flush(&mut self) -> () {
        let mut string_to_be_written = String::new();
        self.buffer.iter().for_each(|element| {
            let string_repr: String = format!("{}", element);
            string_to_be_written.push_str(&string_repr);
            string_to_be_written.push_str("\n");
        });
        self.file_handle
            .write(string_to_be_written.as_bytes())
            .unwrap();
    }
}

impl<T: Display> OutputStream for BufferedOutputStream<T> {
    type Item = T;

    fn write(&mut self, element: Self::Item) -> () {
        self.buffer.push(element);
        if self.buffer.len() == self.buffer.capacity() {
            self.flush();
            self.buffer.clear();
        }
    }

    fn close(&mut self) -> () {
        self.flush();
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
        BufferedOutputStream::create(output_file_path.clone(), 10).write(1);
        assert_eq!(output_file_path.exists(), true);
    }

    #[test]
    fn write_arbitrary_data_to_stream() {
        let tmp_dir = TempDir::new().unwrap();
        let buf = tmp_dir.path().join("opFile.txt");
        let mut stream = BufferedOutputStream::create(buf.as_path(), 10);
        stream.write("Hello");
        stream.close();
    }
}
