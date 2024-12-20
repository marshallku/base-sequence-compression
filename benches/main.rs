use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use base_sequence_compression::{compress_fasta, decompress_fasta};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Small FASTA file", |b| {
        b.iter(|| {
            let input_path = Path::new("benches/input/test.fasta");
            let content = black_box(std::fs::read_to_string(input_path).unwrap());
            let content = compress_fasta(black_box(&content));
            // Save file
            let output_path = Path::new("benches/output/test.bin");
            std::fs::write(output_path, content).unwrap();
        })
    });
    c.bench_function("Large FASTA file", |b| {
        b.iter(|| {
            let input_path = Path::new("benches/input/large.fasta");
            let content = black_box(std::fs::read_to_string(input_path).unwrap());
            let content = compress_fasta(black_box(&content));
            // Save file
            let output_path = Path::new("benches/output/large.bin");
            std::fs::write(output_path, content).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
