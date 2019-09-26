#[macro_use]
extern crate criterion;
extern crate external_merge_sort;
extern crate rand;
extern crate tempfile;

use criterion::Criterion;
use rand::Rng;
use tempfile::TempDir;

use external_merge_sort::io_streams::OutputStream;
use external_merge_sort::one_element_at_a_time::SimpleOutputStream;
use external_merge_sort::stream::system_buffered_io::SystemBufferedOutputStream;

fn generate_random(size: i64) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..size {
        vec.push(rng.gen());
    }
    return vec;
}

pub fn element_at_a_time_output_benchmark(c: &mut Criterion) {
    let random_inputs = generate_random(10000);
    let tmp_dir = TempDir::new().unwrap();
    let benchmark_file = tmp_dir.path().join("benchmark_simple_op.txt");
    let mut output_stream = SimpleOutputStream::create(benchmark_file);
    c.bench_function("element at a time output", |b| {
        b.iter(|| create_stream_and_write(&mut output_stream, &random_inputs))
    });
}

pub fn sys_buffered_output_benchmark(c: &mut Criterion) {
    let random_inputs = generate_random(10000);
    let tmp_dir = TempDir::new().unwrap();
    let benchmark_file = tmp_dir.path().join("benchmark_sys_buffered_op.txt");
    let mut output_stream = SystemBufferedOutputStream::create(benchmark_file);
    c.bench_function("sys buffered output", |b| {
        b.iter(|| create_stream_and_write(&mut output_stream, &random_inputs))
    });
}

fn create_stream_and_write<T: OutputStream>(stream: &mut T, data: &Vec<i32>) -> () {
    for element in data.iter() {
        stream.write(element)
    }
    stream.close()
}

criterion_group!(
    benches,
    element_at_a_time_output_benchmark,
    sys_buffered_output_benchmark
);
criterion_main!(benches);
