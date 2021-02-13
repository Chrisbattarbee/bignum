use criterion::{criterion_group, criterion_main, Criterion, BatchSize, BenchmarkId};
use bignum_rust_lib::BigNum;

macro_rules! generate_big_num_function {
    ( $c: expr, $benchmark_name: expr, $benchmark: expr) => {
        let mut group = $c.benchmark_group($benchmark_name);
        for num_digits in (1..100).step_by(5) {
            group.bench_with_input(
                BenchmarkId::from_parameter(num_digits),
                &num_digits,
                |b, &num_digits| {
                    b.iter_batched(
                        || [
                            BigNum::from_string(String::from("9").repeat(num_digits)),
                            BigNum::from_string(String::from("9").repeat(num_digits)),
                        ],
                        |data| $benchmark(data),
                        BatchSize::SmallInput
                    )
                }
            );
        }
        for num_digits in (100..1000).step_by(100) {
            group.bench_with_input(
                BenchmarkId::from_parameter(num_digits),
                &num_digits,
                |b, &num_digits| {
                    b.iter_batched(
                        || [
                            BigNum::from_string(String::from("9").repeat(num_digits)),
                            BigNum::from_string(String::from("9").repeat(num_digits)),
                        ]
                        ,
                        |data| $benchmark(data),
                        BatchSize::SmallInput
                    )
                }
            );
        }
        group.finish();
    }
}

pub fn add_bignum(c: &mut Criterion) {
    generate_big_num_function!(
    c,
    "Add bignums",
    |data: [BigNum; 2]| {&data[0] + &data[1]}
    );
}

pub fn sub_bignum(c: &mut Criterion) {
    generate_big_num_function!(
    c,
    "Subtract bignums",
    |data: [BigNum; 2]| {&data[0] - &data[1]}
    );
}

pub fn mul_bignum(c: &mut Criterion) {
    generate_big_num_function!(
    c,
    "Multiply bignums",
    |data: [BigNum; 2]| {&data[0] * &data[1]}
    );
}

pub fn div_bignum(c: &mut Criterion) {
    generate_big_num_function!(
    c,
    "Divide bignums",
    |data: [BigNum; 2]| {&data[0] / &data[1]}
    );
}

criterion_group!(benches, add_bignum);
criterion_main!(benches);