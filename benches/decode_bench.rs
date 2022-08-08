use float::float32;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};


fn decode_benchmark(c: &mut Criterion) {
    let test_cases = black_box(
    [42.23,
    321342222.22343,
    ]);
    for &f in test_cases.iter() {
        c.bench_function("decode float number",|b| b.iter(|| float32::Float32::decode(f))
    );

    }
}

criterion_group!(benches,decode_benchmark);
criterion_main!(benches);
