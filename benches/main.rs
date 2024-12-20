use std::{fs::File, io::Read, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use base_sequence_compression::{compress_fasta, decompress_fasta};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Compress Small FASTA file", |b| {
        b.iter(|| {
            let input_path = Path::new("benches/input/test.fasta");
            let content = black_box(std::fs::read_to_string(input_path).unwrap());
            compress_fasta(black_box(&content));
        })
    });
    c.bench_function("Compress Large FASTA file", |b| {
        b.iter(|| {
            let input_path = Path::new("benches/input/large.fasta");
            let content = black_box(std::fs::read_to_string(input_path).unwrap());
            compress_fasta(black_box(&content));
        })
    });
    c.bench_function("Decompress Small FASTA file", |b| {
        b.iter(|| {
            let path = Path::new("benches/output/test.bin");
            let mut file = File::open(path).ok().unwrap();
            let mut content = Vec::new();
            file.read_to_end(black_box(&mut content)).ok();
            decompress_fasta(black_box(&content));
        })
    });
    c.bench_function("Decompress Large FASTA file", |b| {
        b.iter(|| {
            let path = Path::new("benches/output/large.bin");
            let mut file = File::open(path).ok().unwrap();
            let mut content = Vec::new();
            file.read_to_end(black_box(&mut content)).ok();
            decompress_fasta(black_box(&content));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
