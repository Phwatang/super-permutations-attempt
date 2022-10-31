use super_permutations_attempt::{bruteforce, bruteforce_optimise};
use super_permutations_attempt::base::SuperPermHandling;


use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};


fn bruteforce_test(c: &mut Criterion) {
    let n: usize = 5;
    let superperm = black_box(bruteforce::Handle::create_superperm(n));

    let mut bruteforce_group = c.benchmark_group("Brute Force");
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
    let n: usize = 5;
    let superperm = black_box(bruteforce::Handle::create_superperm(n));

    let mut bruteforce_optimise_group = c.benchmark_group("Brute Force Optimised");
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