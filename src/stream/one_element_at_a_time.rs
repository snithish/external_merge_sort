use std::fs::File;

use stream::io_streams::InputStream;

pub struct SimpleStream<T: 'static> {
    file: File,
    val: Vec<T>,
}

impl<T> InputStream<T> for SimpleStream<T> {
    fn open(&self, filename: &str) -> Box<dyn InputStream<T>> {
        let file_handle = File::create(filename).unwrap();
        Box::new(SimpleStream {
            file: file_handle,
            val: vec![],
        })
    }

    fn read_next(&self) -> T {
        unimplemented!()
    }

    fn end_of_stream(&self) -> bool {
        unimplemented!()
    }
}
