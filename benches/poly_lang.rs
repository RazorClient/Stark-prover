use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stark_101::fields::FieldElement;
use stark_101::polynomial::interpolation::{gen_lagrange_polynomials_parallel,interpolate_lagrange_polynomials};
use stark_101::polynomial::Polynomial;


/// Benchmark Lagrange polynomial generation
fn bench_lagrange_polynomials(c: &mut Criterion) {
    // We can benchmark multiple sizes of n in a loop or separate benchmarks.
    let sizes = [10, 50, 100, 200, 500];

    for &size in &sizes {
        // Prepare a group so Criterion can plot each size separately
        let mut group = c.benchmark_group("gen_lagrange_polynomials:");

        // Generate random x values
        let xs: Vec<FieldElement<7>> = (0..size).map(|_| FieldElement::<7>::random()).collect();
        
        // Benchmark a closure that calls `gen_lagrange_polynomials`
        group.bench_function(format!(" n={}", size), |b| {
            b.iter(|| {
                // Use black_box(...) to prevent compiler optimizations
                black_box(gen_lagrange_polynomials_parallel(black_box(&xs)));
            });
        });

        group.finish();
    }
}

/// Benchmark Lagrange interpolation

fn bench_interpolate_lagrange(c: &mut Criterion) {
    let sizes = [10, 50, 100, 200, 500];

    for &size in &sizes {
        let mut group = c.benchmark_group("interpolate_lagrange_polynomials");

        // Generate random (xs, ys) pairs using the implemented `random()`
        let xs: Vec<FieldElement<7>> = (0..size).map(|_| FieldElement::<7>::random()).collect();
        let ys: Vec<FieldElement<7>> = (0..size).map(|_| FieldElement::<7>::random()).collect();

        group.bench_function(format!("n={}", size), |b| {
            b.iter(|| {
                black_box(interpolate_lagrange_polynomials(black_box(&xs), black_box(&ys)));
            });
        });

        group.finish();
    }
}


// The Criterion macros that define the main entry point for `cargo bench`
criterion_group!(
    benches,
    bench_lagrange_polynomials,
    bench_interpolate_lagrange
);
criterion_main!(benches);
