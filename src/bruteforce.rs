#![allow(dead_code)]
// Functions to generate and check a superperm with nieve/ignorant
// bruteforce methods. Absolutely no optimisations have been made
use itertools::Itertools;


/// Return a vec containing all possible permutations of the sequence [1..n]
/// E.g generate_perms(2) = [[1,2], [2,1]]
fn generate_perms(n: usize) -> Vec<Vec<usize>> {
    let mut permuts: Vec<Vec<usize>> = Vec::new();
    let iter = (1..n+1).permutations(n);
    for perm in iter {
        permuts.push(perm);
    }
    return permuts;
}

/// Check if a vector of numbers is a valid superpermutation for a sequence of [1..perm_n]
/// E.g check_superperm([1,2,1], 2) will check if [1,2,1] is a superperm of [1,2]
pub fn check_superperm(potential_super: &Vec<usize>, perm_n: usize) -> bool {
    // Brute force method
    let perms = generate_perms(perm_n);
    // Create boolean for each perm to check
    let mut perm_checklist: Vec<bool> = vec![false; perms.len()];
    // Keep slicing over potential_super and check if the slice is a permutation
    for i in 0..potential_super.len()-perm_n+1 {
        let slice = &potential_super[i..i+perm_n];
        for (pos, perm) in perms.iter().enumerate() {
            if perm == slice {
                perm_checklist[pos] = true;
            }
        }
    }
    // Check if all permutations have been sliced over/seen
    for element in perm_checklist {
        if element == false {
            return false;
        }
    }
    // If function made it this far then potential_super is a superperm
    return true;
}

/// Returns a valid superpermutation for the sequence of [1..perm_n]
/// Does not guarantee minimality for the superpermutation returned
/// E.g create_superperm(3) = [1,2,3,1,2,1,3,2,1]
pub fn create_superperm(perm_n: usize) -> Vec<usize> {
    let mut superperm: Vec<usize> = Vec::new();
    let all_perms = generate_perms(perm_n);
    let mut perm_checklist: Vec<bool> = vec![false; all_perms.len()];
    // Set an initial sequence to superperm before starting algo
    superperm.append(&mut all_perms[0].clone());
    perm_checklist[0] = true;
    // Loop for n possible permutations
    for _ in 0..all_perms.len() {
        // Loop to grab biggest trailing size then smallest
        for i in (0..perm_n).rev() {
            let mut perm_matched = false;
            // Retrieve a slice of the trailing elements in superperm
            let trailing = &superperm.clone()[superperm.len()-i..];
            // Check if trailing equals the starting of any perms left to be checked off
            for (pos, perm) in all_perms.iter().enumerate() {
                if &perm[0..i] == trailing && perm_checklist[pos] == false {
                    // Check off the perm and append on the rest of the perm onto the superperm
                    perm_matched = true;
                    perm_checklist[pos] = true;
                    superperm.extend_from_slice(&perm[i..]);
                    break;
                }
            }
            if perm_matched {
                break;
            }
            // If loop reaches this point, trailing sequence didn't match the start of any unchecked
            // perms. Next loop round, the size of the trailing sequence gets smaller.
            // When i = 0, the trailing sequence will be empty []. In this case the first unchecked
            // permutation will get appended onto the superperm fully
        }

        // Reaching this point, one permutation is guaranteed to be check off
    }
    return superperm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_check() {
        for n in 1..6 {
            assert!(check_superperm(&create_superperm(n), n));
        }
    }
}
