use criterion::{criterion_group, criterion_main, Criterion};

fn stack_variable(c: &mut Criterion) {
    c.bench_function("stack integer", |b| {
        b.iter(|| {
            let x = 42;
            x
        })
    });
}

fn heap_string(c: &mut Criterion) {
    c.bench_function("heap string", |b| {
        b.iter(|| {
            let s = String::from("Rust");
            s
        })
    });
}

fn constant_variable(c: &mut Criterion) {
    c.bench_function("constant int variable", |b| {
        b.iter(|| {
            const X: i32 = 42;
            X
        })
    });

    c.bench_function("constant str variable", |b| {
        b.iter(|| {
            const X: &str = "42";
            X
        })
    });

    // not allowed
    // c.bench_function("constant STring variable", |b| {
    //     b.iter(|| {
    //         const X: String = "42".to_string();
    //         X
    //     })
    // });
}

fn clone_string(c: &mut Criterion) {
    c.bench_function("clone string", |b| {
        let s1 = String::from("Rust");
        b.iter(|| s1.clone())
    });
}

fn reference_borrow(c: &mut Criterion) {
    c.bench_function("reference borrow", |b| {
        let s1 = String::from("Rust");
        b.iter(|| {
            let s2 = &s1;
            s2
        })
    });
}

fn shadowing(c: &mut Criterion) {
    c.bench_function("heap shadowing", |b| {
        b.iter(|| {
            let s1 = String::from("Rust");
            let s2 = s1;
            s2
        })
    });

    c.bench_function("stack shadowing", |b| {
        b.iter(|| {
            let s1 = 100;
            let s2 = s1;
            s2
        })
    });
}

fn testing_iter(c: &mut Criterion) {
    c.bench_function("iter: ", |b| {
        let s1 = vec![String::from("still"), String::from("1234567"), String::from("Mananaf"), String::from("Aisha")];;
        b.iter(|| {
            let s2 = longest_string(&s1);
            s2
        })
    });

    c.bench_function("function: ", |b| {
        let s1 = vec![String::from("still"), String::from("1234567"), String::from("Mananaf"), String::from("Aisha")];;
        b.iter(|| {
            let mut longest = "".to_string();
            for item in &s1 {
                if item.len() > longest.len() {
                    longest = item.clone()
                }
            }
            longest
        })
    });
}

fn longest_string(array_of_strings: &[String]) -> &String {
    array_of_strings.iter().max_by_key(|s| s.len()).unwrap()
}

criterion_group!(benches, stack_variable, heap_string, clone_string, reference_borrow, testing_iter, constant_variable, shadowing);
criterion_main!(benches);
