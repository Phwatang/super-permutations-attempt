use super_permutations_attempt::{bruteforce, bruteforce_optimise};
use super_permutations_attempt::base::SuperPermHandling;
use rand::Rng;

/// Common tests to perform on each implementation
fn common_checks(checker: fn(&Vec<usize>, usize) -> bool, creator: fn(usize) -> Vec<usize>) {
    // self agreement test
    for n in 1..7 {
        assert!(checker(&creator(n), n));
    }

    // invalid case
    assert!(!checker(&vec![1,2,3,2], 3));
    // valid cases
    assert!(checker(&vec![1,2,3,1,2,1,3,2,1], 3));
    assert!(checker(&vec![1,2,3,4,1,2,3,1,4,2,3,1,2,4,3,1,2,1,3,4,2,1,3,2,4,1,3,2,1,4,3,2,1], 4));
    // fuzz testing valid cases (appending random numbers at beginning/end
    // should not affect a valid superpermutation)
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let mut pre = vec![0; rng.gen_range(1..10)];
        let mut suf = vec![0; rng.gen_range(1..10)];
        pre.iter_mut().for_each(|x| *x = rng.gen_range(1..4));
        suf.iter_mut().for_each(|x| *x = rng.gen_range(1..4));
        let vec = [pre, vec![1,2,3,1,2,1,3,2,1], suf].concat();
        assert!(checker(&vec, 3));
    }

}

#[test]
fn bruteforce_test() {
    common_checks(bruteforce::Handle::check_superperm, bruteforce::Handle::create_superperm);
}

#[test]
fn bruteforce_optimise_test() {
    common_checks(bruteforce_optimise::Handle::check_superperm, bruteforce_optimise::Handle::create_superperm);
}
