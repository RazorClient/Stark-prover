#![cfg_attr(feature = "nightly", feature(concat_idents))]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;


use stark_101::fields::FieldElement;
use stark_101::polynomial::Polynomial;


const PRIME_MODULI: &[u64] = &[
    7,           // Very small
    17,          // Small
    1_000_003,   // Medium (~1e6)
    998_244_353, // Large (common FFT prime)
];

macro_rules! define_benches_for_modulus {
    ($modulus:expr, $mod_name:ident) => {
        mod $mod_name {
            use super::*;
            type FE = FieldElement<$modulus>;
            type Poly = Polynomial<$modulus>;

            /// Generate a random FieldElement
            //use a cryto safe random no generator
            fn random_fe(rng: &mut ChaCha20Rng) -> FE {
                let val = rng.next_u64() % $modulus;
                FE::new(val)
            }

            /// Generate a random Polynomial<$modulus> of given degree
            fn random_poly(rng: &mut ChaCha20Rng, degree: usize) -> Poly {
                let coeffs = (0..=degree).map(|_| random_fe(rng)).collect();
                Poly::new(coeffs)
            }

            /// Benchmark Polynomial Addition
            pub fn bench_add(c: &mut Criterion) {
                let mut group = c.benchmark_group(concat!("Add_", stringify!($modulus)));
                group.sampling_mode(SamplingMode::Flat);

                let sizes = [10, 100, 1_000];
                let mut rng = ChaCha20Rng::seed_from_u64(42);

                for &size in &sizes {
                    let p1 = random_poly(&mut rng, size);
                    let p2 = random_poly(&mut rng, size);

                    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
                        b.iter(|| {
                            let result = black_box(&p1) + black_box(&p2);
                            black_box(result);
                        })
                    });
                }

                group.finish();
            }

            // /// Benchmark Polynomial Subtraction
            // pub fn bench_sub(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("Sub_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(12345);

            //     for &size in &sizes {
            //         let p1 = random_poly(&mut rng, size);
            //         let p2 = random_poly(&mut rng, size);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 let result = black_box(&p1) - black_box(&p2);
            //                 black_box(result);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial Multiplication
            // pub fn bench_mul(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("Mul_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(9999);

            //     for &size in &sizes {
            //         let p1 = random_poly(&mut rng, size);
            //         let p2 = random_poly(&mut rng, size);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 let result = black_box(&p1) * black_box(&p2);
            //                 black_box(result);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial Division with Remainder
            // pub fn bench_div_rem(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("DivRem_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(7777);

            //     for &size in &sizes {
            //         let p1 = random_poly(&mut rng, size);
            //         let divisor_size = (size / 2).max(1);
            //         let mut p2 = random_poly(&mut rng, divisor_size);

            //         // Ensure the divisor is not the zero polynomial
            //         if p2.is_zero() {
            //             p2 = Poly::new(vec![FE::one()]);
            //         }

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 let (q, r) = black_box(&p1).div_rem(black_box(&p2));
            //                 black_box((q, r));
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial Composition
            // pub fn bench_compose(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("Compose_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 50, 100];
            //     let mut rng = ChaCha20Rng::seed_from_u64(5555);

            //     for &size in &sizes {
            //         let p1 = random_poly(&mut rng, size);
            //         let p2_size = (size / 2).max(1);
            //         let p2 = random_poly(&mut rng, p2_size);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 let result = black_box(&p1).compose(black_box(&p2));
            //                 black_box(result);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial Evaluation
            // pub fn bench_eval(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("Eval_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000, 5_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(3333);

            //     for &size in &sizes {
            //         let p = random_poly(&mut rng, size);
            //         let x = random_fe(&mut rng);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 let result = black_box(&p).evaluate(black_box(x));
            //                 black_box(result);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial AddAssign
            // pub fn bench_add_assign(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("AddAssign_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(4040);

            //     for &size in &sizes {
            //         let mut p1 = random_poly(&mut rng, size);
            //         let p2 = random_poly(&mut rng, size);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 p1.add_assign(black_box(&p2));
            //                 black_box(&p1);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            // /// Benchmark Polynomial MulAssign
            // pub fn bench_mul_assign(c: &mut Criterion) {
            //     let mut group = c.benchmark_group(concat!("MulAssign_", stringify!($modulus)));
            //     group.sampling_mode(SamplingMode::Flat);

            //     let sizes = [10, 100, 1_000];
            //     let mut rng = ChaCha20Rng::seed_from_u64(5050);

            //     for &size in &sizes {
            //         let mut p1 = random_poly(&mut rng, size);
            //         let p2 = random_poly(&mut rng, size);

            //         group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            //             b.iter(|| {
            //                 p1.mul_assign(black_box(&p2));
            //                 black_box(&p1);
            //             })
            //         });
            //     }

            //     group.finish();
            // }

            /// Register all benchmark functions for this modulus
            criterion_group!(
                $mod_name,
                bench_add,
                // bench_sub,
                // bench_mul,
                // bench_div_rem,
                // bench_compose,
                // bench_eval,
                // bench_add_assign,
                // bench_mul_assign
            );

            criterion_main!($mod_name);
        }
    }}

define_benches_for_modulus!(17, benches_17);
// define_benches_for_modulus!(17,benches_17);
