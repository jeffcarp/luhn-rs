use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_validate(c: &mut Criterion) {
    let isin = "4111111111111111";
    c.bench_function("validate", |b| b.iter(|| luhn::valid(black_box(isin))));
}

fn bench_checksum(c: &mut Criterion) {
    let s = "111111118";
    c.bench_function("checksum", |b| {
        b.iter(|| luhn::checksum(black_box(s.as_bytes())))
    });
}

criterion_group!(benches, bench_validate, bench_checksum);
criterion_main!(benches);
