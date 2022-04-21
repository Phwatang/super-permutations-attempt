mod bruteforce_no_optimise;
mod bruteforce_optimise;
use std::time::Instant;

fn main() {
    let n: usize = 7;
    let mut before = Instant::now();
    bruteforce_no_optimise::test(n);
    println!("bruteforce_no_optimise time elapsed: {:.3?}", before.elapsed());
    before = Instant::now();
    bruteforce_optimise::test(n);
    println!("bruteforce_optimise time elapsed: {:.3?}", before.elapsed());
}
