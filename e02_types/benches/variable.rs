use criterion::{criterion_group, criterion_main, Criterion};

fn stack_variable(c: &mut Criterion) {
    c.bench_function("stack integer", |b| {
        b.iter(|| {
            let x: u64 = 42000000000000000;
            x
        })
    });
}

fn heap_variable(c: &mut Criterion) {
    c.bench_function("heap string", |b| {
        b.iter(|| {
            let x: String = String::from("420000000000000000000000000000000000000000000000000");
            x
        })
    });
}

criterion_group!(benches, stack_variable, heap_variable);
criterion_main!(benches);
