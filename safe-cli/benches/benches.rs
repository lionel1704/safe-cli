#[macro_use]
extern crate duct;
use std::fs;

use criterion::{BatchSize, Criterion};
// use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TEST_FILE_RANDOM_CONTENT: &str = "test_file_random_content.txt";

const SAMPLE_SIZE: usize = 20;

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(SAMPLE_SIZE)
}
fn main() {
    let mut criterion = custom_criterion();

    bench_cli_put(&mut criterion);
}

fn put_random_content() -> Result<(), String> {
    let random_content: String = (0..10).map(|_| rand::random::<char>()).collect();
    fs::write(TEST_FILE_RANDOM_CONTENT, random_content)
        .map_err(|_| "Error writing random content".to_string())?;
    Ok(())
}

// #[criterion(custom_criterion())]
fn bench_cli_put(c: &mut Criterion) {
    c.bench_function("cli put random data", |b| {
        b.iter_batched(
            put_random_content,
            |_| {
                //  use the safe command, so for bench it has to be installed
                cmd!("safe", "files", "put", TEST_FILE_RANDOM_CONTENT)
                    .read()
                    .unwrap()
            },
            BatchSize::SmallInput,
        )
    });
}
