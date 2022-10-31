#[path="../src/bruteforce.rs"]
mod bruteforce;

#[path="../src/bruteforce_optimise.rs"]
mod bruteforce_optimise;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};


fn bruteforce_test(c: &mut Criterion) {
    let n: usize = 6;
    let superperm = black_box(bruteforce::create_superperm(n));

    let mut bruteforce_group = c.benchmark_group("Brute Force");
    bruteforce_group.bench_function(
        "Creation", 
        |b| b.iter(|| bruteforce::create_superperm(n))
    );
    bruteforce_group.bench_function(
        "Checking", 
        |b| b.iter(|| bruteforce::check_superperm(&superperm, n))
    );
    bruteforce_group.finish();

}

fn bruteforce_optimise_test(c: &mut Criterion) {
    let n: usize = 5;
    let superperm = black_box(bruteforce::create_superperm(n));

    let mut bruteforce_optimise_group = c.benchmark_group("Brute Force Optimised");
    let p = bruteforce_optimise::PermutationsHelper::new(n);
    bruteforce_optimise_group.bench_function(
        "Creation",
        |b| b.iter(|| p.create_superperm())
    );
    bruteforce_optimise_group.bench_function(
        "Checking", 
        |b| b.iter(|| p.check_superperm(&superperm))
    );
    bruteforce_optimise_group.finish();
}

criterion_group!(benches, bruteforce_optimise_test);
criterion_main!(benches);