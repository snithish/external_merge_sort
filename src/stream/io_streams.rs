use std::fmt::Display;
use std::path::PathBuf;

pub trait InputStream<T> {
    fn open(&self, file_path: impl Into<PathBuf>) -> std::io::Result<()>;
    fn read_next(&self) -> T;
    fn end_of_stream(&self) -> bool;
}

pub trait OutputStream {
    type Item: Display;
    fn write(&mut self, element: Self::Item) -> ();
    fn close(&mut self) -> ();
}
