use super_permutations_attempt::{bruteforce, bruteforce_optimise};
use super_permutations_attempt::base::SuperPermHandling;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

const N: usize = 5;

fn common_bench(c: &mut Criterion, superperm_h: impl SuperPermHandling, group_name: &String) {
    let n: usize = black_box(N);
    let superperm = black_box(superperm_h.create_superperm(n));
    let mut bruteforce_group = c.benchmark_group(group_name);

    bruteforce_group.bench_function(
        "creation",
        |b| b.iter(|| superperm_h.create_superperm(n))
    );
    bruteforce_group.bench_function(
        "checking",
        |b| b.iter(|| superperm_h.check_superperm(&superperm, n))
    );    
    bruteforce_group.finish();
}

fn bruteforce_bench(c: &mut Criterion) {
    common_bench(c, bruteforce::Handle{}, &String::from("bruteforce"));
}

fn bruteforce_optimise_bench(c: &mut Criterion) {
    common_bench(c, bruteforce_optimise::Handle{}, &String::from("bruteforce_optimise"));
}

criterion_group!(benches, bruteforce_bench, bruteforce_optimise_bench);
criterion_main!(benches);