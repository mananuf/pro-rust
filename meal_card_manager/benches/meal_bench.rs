use criterion::{criterion_group, criterion_main, Criterion, black_box};
use meal_card_manager::{
    student::Student, types::{Amount, CampusMealSystem, MealCard, TransactionType}
};

pub fn bench_student_registration(c: &mut Criterion) {
    c.bench_function("register 10k students", |b| {
        b.iter(|| {
            let mut system = CampusMealSystem::new();

            for i in 0..10_000 {
                system.register_student(black_box("mano"), black_box(i));
            }
        })
    });
}

pub fn bench_credit_transaction(c: &mut Criterion) {
    let mut system = CampusMealSystem::new();
    let id = system.register_student("mano", 200);

    c.bench_function("credit transaction", |b| {
        b.iter(|| {
            system
                .process_transaction(id, black_box(TransactionType::Credit(50)))
                .unwrap();
        })
    });
}

pub fn bench_debit_transaction(c: &mut Criterion) {
        c.bench_function("debit transaction", |b| {
        b.iter_batched(
            || Student::builder("name", 200).meal_card(MealCard { balance: Amount(10000)}).build(),
            |mut student| {
                black_box(
                    student
                        .apply_transaction(TransactionType::Debit(5))
                        .unwrap(),
                );
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn bench_bulk_transactions(c: &mut Criterion) {
    let mut system = CampusMealSystem::new();
    let id = system.register_student("mano", 200);

    c.bench_function("10k mixed transactions", |b| {
        b.iter(|| {
            for _ in 0..10_000 {
                let _ = system.process_transaction(id, TransactionType::Credit(1));
                let _ = system.process_transaction(id, TransactionType::Debit(1));
            }
        })
    });
}

pub fn bench_student_lookup(c: &mut Criterion) {
    let mut system = CampusMealSystem::new();
    
    for i in 0..50_000 {
        system.register_student("mano", i);
    }

    c.bench_function("lookup student by id", |b| {
        b.iter(|| {
            let _ = system.get_student(black_box(25_000));
        })
    });
}

criterion_group!(
    benches,
    bench_student_registration,
    bench_credit_transaction,
    bench_debit_transaction,
    bench_bulk_transactions,
    bench_student_lookup,
);
criterion_main!(benches);
