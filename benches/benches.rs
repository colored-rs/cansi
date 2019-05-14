#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create terminal string default", |b| {
        let term = MemoryTerminal::default();
        b.iter(|| create_terminal_string(term))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
