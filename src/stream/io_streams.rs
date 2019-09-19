pub trait InputStream<T> {
    fn open(&self, filename: &str) -> Box<dyn InputStream<T>>;
    fn read_next(&self) -> T;
    fn end_of_stream(&self) -> bool;
}

pub trait OutputStream<T> {
    fn create(&self, filename: &str) -> ();
    fn write(&self, element: T) -> ();
    fn close(&self) -> ();
}
