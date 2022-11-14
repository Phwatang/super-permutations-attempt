use super_permutations_attempt::{bruteforce, bruteforce_optimise};
use super_permutations_attempt::base::SuperPermHandling;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

const N: usize = 5;

fn bruteforce_test(c: &mut Criterion) {
    let n: usize = black_box(N);
    let superperm = black_box(bruteforce::Handle::create_superperm(n));
    
    let mut bruteforce_group = c.benchmark_group("bruteforce");
    bruteforce_group.bench_function(
        "Creation", 
        |b| b.iter(|| bruteforce::Handle::create_superperm(n))
    );
    bruteforce_group.bench_function(
        "Checking", 
        |b| b.iter(|| bruteforce::Handle::check_superperm(&superperm, n))
    );
    bruteforce_group.finish();

}

fn bruteforce_optimise_test(c: &mut Criterion) {
    let n: usize = black_box(N);
    let superperm = black_box(bruteforce::Handle::create_superperm(n));

    let mut bruteforce_optimise_group = c.benchmark_group("bruteforce_optimise");
    bruteforce_optimise_group.bench_function(
        "Creation",
        |b| b.iter(|| bruteforce_optimise::Handle::create_superperm(n))
    );
    bruteforce_optimise_group.bench_function(
        "Checking", 
        |b| b.iter(|| bruteforce_optimise::Handle::check_superperm(&superperm, n))
    );
    bruteforce_optimise_group.finish();
}

criterion_group!(benches, bruteforce_test, bruteforce_optimise_test);
criterion_main!(benches);