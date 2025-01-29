// benches/field_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stark_101::fields::FieldElement; // Update this if your crate name is different

fn bench_field_operations(c: &mut Criterion) {
    const MODULUS: u64 = 7;

    let a_el = FieldElement::<MODULUS>::new(3);
    let b_el = FieldElement::<MODULUS>::new(5);

    c.bench_function("field_addition", |bencher| {
        bencher.iter(|| {
            black_box(a_el + b_el);
        })
    });

    c.bench_function("field_multiplication", |bencher| {
        bencher.iter(|| {
            black_box(a_el * b_el);
        })
    });

    c.bench_function("field_exponentiation", |bencher| {
        bencher.iter(|| {
            black_box(a_el.pow(10));
        })
    });

    c.bench_function("field_inverse", |bencher| {
        bencher.iter(|| {
            black_box(a_el.inverse());
        })
    });
}


criterion_group!(benches, bench_field_operations);
criterion_main!(benches);