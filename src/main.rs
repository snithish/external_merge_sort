extern crate rand;

use std::fs::File;
use std::io::Write;

use rand::Rng;

fn main() -> std::io::Result<()> {
    let mut file = File::create("generated_files/one.txt")?;
    let random_integers = generate_random(1000000);
    for random_integer in random_integers.iter() {
        let string_repr: String = format!("{}", random_integer);
        let escape_line = string_repr + "\n";
        file.write(escape_line.as_bytes())?;
    }
    Ok(())
}

fn generate_random(size: i64) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _i in 0..size {
        vec.push(rng.gen());
    }
    return vec;
}
