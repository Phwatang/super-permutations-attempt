#![allow(dead_code)]
// Same goals and fundamental algorithims as bruteforce_no_optimise
// except optimisations have been made using better data structuring.

use crate::base::*;

/// Class to encode any value into a different base where
/// each "position" can be of a different base
pub struct MixedRadix {
    /// The base of each position for this system
    pub bases: Vec<usize>,
    /// The first integer above 0 that is unrepresentable with the bases given
    pub max_value: usize,
}

/// Short for "Mixed Radix Representation".
/// Data type for a number represented in a mixed radix system
type MixedRadixRepr = Vec<usize>;

impl MixedRadix {
    /// bases parameter details:
    ///  - Vec passed in expected to be the base for each position,
    ///  - Values in the front of vec refer to least significant positions.
    ///  - It also indirectly sets the number of positions available.
    pub fn new(bases: Vec<usize>) -> MixedRadix{
        // Calculate maximum representable value with the bases passed in
        let mut max: usize = 1;
        for base in &bases {
            max *= base;
        }
        return MixedRadix {
            bases: bases,
            max_value: max,
        }
    }

    /// Interprets a value into a representation of the bases specified at instantiation
    /// 
    /// E.g The value 54 encoded to the bases (5, 4, 3) will be represented as (4, 2, 2)
    pub fn encode_value(&self, val: &usize) -> MixedRadixRepr {
        let mut representation: MixedRadixRepr = vec![0; self.bases.len()];
        let mut carry_over = val.clone();
        for (i, base) in self.bases.iter().enumerate() {
            representation[i] = carry_over % base;
            carry_over = carry_over / base;
        }
        return representation;
    }
    /// Interprets a representation into a value. Inverse of encode_value
    pub fn decode_representation(&self, repr: &MixedRadixRepr) -> usize {
        let mut sum: usize = 0;
        let mut position_mult: usize = 1;
        for (pos, base) in self.bases.iter().enumerate() {
            sum += repr[pos] * position_mult;
            position_mult *= base;
        }
        return sum;
    }
}

// implementing iteration over MixedRadix
// Used: https://stackoverflow.com/questions/68606470/how-to-return-a-reference-when-implementing-an-iterator
pub struct MixedRadixIter<'a> {
    system: &'a MixedRadix,
    i: usize,
}
impl<'a> Iterator for MixedRadixIter<'a> {
    type Item = MixedRadixRepr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.system.max_value {
            return None;
        } else {
            let output = self.system.encode_value(&self.i);
            self.i += 1;
            return Some(output);
        }
    }
}
impl<'a> IntoIterator for &'a MixedRadix {
    type Item = MixedRadixRepr;
    type IntoIter = MixedRadixIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MixedRadixIter {
            system: self,
            i: 0,
        }
    }
}


/// Class to help with the handling of permutations for a sequence of the form: (1,2,3,..,n)
pub struct PermutationsHelper {
    pub sequence_n: usize,
    pub core_sequence: Vec<usize>,
    pub base_encoder: MixedRadix,
}
impl PermutationsHelper {
    // Why use MixedRadix for permutations?
    // Imagine the sequence [1, 2, 3] and all its permutations.
    // When creating a permutation, we have 3 positions to fill up.
    // We insert "1" in one of the three positions. Two positions are left (e.g [_, 1, _])
    // We insert "2" in one of the two positions. One position are left (e.g [2, 1, _])
    // We insert "3" in the final position. Permutation is complete (e.g [2, 1, 3])
    // With this logic, each permutation can be represented as a number in a variable base system.
    // [1, 2, 3] be represented as (0, 0, 0) or 0
    // [2, 1, 3] be represented as (1, 0, 0) or 1
    // [3, 2, 1] be represented as (2, 1, 0) or 5

    /// n sets the upper limit for self.core_sequence.  
    /// 
    /// E.g PermutationsHelper::new(5) will focus on permutations of the sequence (1,2,3,4,5)
    pub fn new(n: usize) -> PermutationsHelper {
        let bases: Vec<usize> = (1..n+1).rev().collect();
        let obj = PermutationsHelper{
            sequence_n: n,
            core_sequence: (1..n+1).collect(),
            base_encoder: MixedRadix::new(bases), 
        };
        return obj;
    }

    /// Convert a VariableBaseRepresentation into a distinct permutation
    pub fn encoded_to_perm(&self, encoded: &MixedRadixRepr) -> Vec<usize> {
        let mut output_perm: Vec<usize> = vec![0; self.sequence_n];
        for (pos, token) in self.core_sequence.iter().enumerate() {
            let shift = encoded[pos];
            let mut ind: usize = 0;

            // Use each number in the encoded Vec to know how much to shift along
            // before inserting.

            // Skip to first non-filled position
            while output_perm[ind] != 0 {
                ind += 1;
            }
            // For each shift
            for _ in 0..shift {
                // Move along 1
                ind += 1;
                // Skip to next non-filled position
                while output_perm[ind] != 0 {
                    ind += 1;
                }
            }
            output_perm[ind] = token.clone();
        }
        return output_perm
    }

    /// Returns a vector of encoded representations for permutations whose 
    /// starting sequence matches perm_target.
    pub fn possible_encodes(&self, perm_target: &Vec<usize>) -> Vec<MixedRadixRepr> {
        let filler = self.sequence_n+1;
        let mut working_space: MixedRadixRepr = vec![filler; self.sequence_n];
        let mut encode_ranges: Vec<std::ops::Range<usize>> = Vec::with_capacity(self.sequence_n);

        // Check for empty perm_target, this means all permutations/encodes fit the target
        if perm_target.len() == 0 {
            return self.base_encoder.into_iter().collect();
        }

        // Calculate the range of encoded values possible that would map to the perm_target
        for (pos, token) in self.core_sequence.iter().enumerate() {
            let mut shifted: usize = 0;
            let mut i: usize = 0;
            let mut matched = false;
            while i < perm_target.len() {
                // auto skip over filled spots
                while working_space[i] != filler {
                    i += 1;
                    // refactor this!!!!
                    if i >= working_space.len() {
                        return vec![];
                    }
                }
                // check for match
                if &perm_target[i] == token {
                    working_space[i] = token.clone();
                    matched = true;
                    break;
                }
                // move to next spot and count it
                shifted += 1;
                i += 1;
            }

            if matched {
                encode_ranges.push(shifted..shifted+1);
            } else {
                if shifted >= self.base_encoder.bases[pos] {
                    return vec![]
                } else {
                    encode_ranges.push(shifted..self.base_encoder.bases[pos]);
                }
            }
        }

        // Go through all the possible encode ranges.
        // Create another VariableBaseSystem and do some conversion to naturally
        // go through all the encode value ranges for each position.
        let system = MixedRadix::new(
            encode_ranges.iter().map(|x| x.end-x.start).collect()
        );
        let mut possibilities: Vec<MixedRadixRepr> = Vec::with_capacity(system.max_value);
        for mut repr in system.into_iter() {
            for (i, num) in repr.iter_mut().enumerate() {
                *num += encode_ranges[i].start;
            }
            possibilities.push(repr);
        }
        return possibilities;
    }

    /// Checks if potential_super is a valid superpermutation of self.core_sequence
    pub fn check_superperm(&self, potential_super: &Vec<usize>) -> bool {
        // Brute force approach
        let mut perm_checklist: Vec<bool> = vec![false; self.base_encoder.max_value];
        // Keep slicing over potential_super and check if the slice is a permutation
        for i in 0..potential_super.len()-self.sequence_n+1 {
            let slice = &potential_super[i..i+self.sequence_n];
            let encodes = self.possible_encodes(&slice.to_vec());
            // encodes may be an empty list thus check with a for loop
            for encode in &encodes {
                perm_checklist[self.base_encoder.decode_representation(encode)] = true;
            }
        }
        // Check if all permutations have been seen
        for element in perm_checklist {
            if element == false {
                return false;
            }
        }
        // If function made it this far then potential_super is a valid superperm
        return true;
    }

    /// Returns a valid superpermutation of self.core_sequence
    pub fn create_superperm(&self) -> Vec<usize> {
        // Set an intial sequence to build the superperm from before starting algo
        let mut superperm: Vec<usize> = (1..self.sequence_n+1).collect();
        let mut perm_checklist: Vec<bool> = vec![false; self.base_encoder.max_value];
        perm_checklist[0] = true;

        // Loop for all possible permutations to be covered
        for _ in 1..self.base_encoder.max_value {
            let mut trail_matched = false;

            // Loop to grab the trailing sequences of superperm
            for i in (1..self.sequence_n).rev() {
                let mut perm_matched = false;
                // Grab the trailing sequence
                let trailing = &superperm[superperm.len()-i..];
                // Check if trailing equals the start of any perms left to be checked off
                for encode in &self.possible_encodes(&trailing.to_vec()) {
                    let value = self.base_encoder.decode_representation(encode);
                    if perm_checklist[value] == false { // Perm has not been checked off
                        // Check off perm and append rest of it onto superperm
                        perm_checklist[value] = true;
                        superperm.extend_from_slice(&self.encoded_to_perm(encode)[i..]);
                        perm_matched = true;
                        break;
                    }
                }
                if perm_matched {
                    trail_matched = true;
                    break;
                }
                // If loop reaches this point, trailing sequence didn't match the start of any unchecked
                // perms. Next loop round, the size of the trailing sequence gets smaller.
                // When i = 0, the trailing sequence will be empty []. In this case the first unchecked
                // permutation will get appended onto the superperm fully
            }

            if trail_matched {
                // No trailing can be used to build off of. We are free to append on an entire permutation onto the super
                for (i, checked) in perm_checklist.iter().enumerate() {
                    if *checked == false {
                        let mut perm = self.encoded_to_perm(&self.base_encoder.encode_value(&i));
                        superperm.append(&mut perm);
                        break
                    }
                }
            }
        }
        return superperm;
    }
}

pub struct Handle;
impl SuperPermHandling for Handle {
    fn check_superperm(sequence: &Vec<usize>, n_tokens: usize) -> bool {
        return PermutationsHelper::new(n_tokens).check_superperm(sequence);
    }

    fn create_superperm(n_tokens: usize) -> Vec<usize> {
        return PermutationsHelper::new(n_tokens).create_superperm();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varbasesys_iteration() {
        let system = MixedRadix::new(vec![3,2,2]);
        let expected = vec![
            vec![0,0,0],
            vec![1,0,0],
            vec![2,0,0],
            vec![0,1,0],
            vec![1,1,0],
            vec![2,1,0],
            vec![0,0,1],
            vec![1,0,1],
            vec![2,0,1],
            vec![0,1,1],
            vec![1,1,1],
            vec![2,1,1],
        ];
        // check if iterator can be used multiple times
        for _ in 0..2 {
            for (i, repr) in system.into_iter().enumerate() {
                assert_eq!(repr, expected[i]);
            }
        }
    }

    #[test]
    fn varbasesys_edgecase() {
        let system = MixedRadix::new(vec![1,2]);
        let expected = vec![
            vec![0,0],
            vec![0,1],
        ];
        for (i, repr) in system.into_iter().enumerate() {
            assert_eq!(repr, expected[i]);
        }
    }
}

