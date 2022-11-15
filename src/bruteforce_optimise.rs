#![allow(dead_code)]
// Same goals and fundamental algorithms as bruteforce_no_optimise
// except optimisations have been made using better data structuring.

// To do: Improve efficiency futher. Only starts to beat bruteforce.rs at
//        for superpermutations of 6~7 tokens. <5 tokens, bruteforce.rs
//        wins by quite a decent margin.

use crate::base::*;

/// Class to encode any value into a different base where
/// each "position" can be of a different base
struct MixedRadix {
    /// The base of each position for this system
    pub bases: Vec<usize>,
    /// The first integer above 0 that is unrepresentable with the bases given
    pub max_value: usize,
}

/// Short for "Mixed Radix Representation".
/// Data type for a value represented in a mixed radix system
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



/// Class to help with the handling of permutations for an arbitrary sequence
struct PermutationMapper {
    pub core_sequence: Vec<usize>,
    pub mixed_radix_sys: MixedRadix,
}
impl PermutationMapper {
    // Why use MixedRadix for permutations?
    // Imagine the sequence [1, 2, 3] and all its permutations.
    // When creating a permutation, we have 3 positions to fill up.
    // We insert "1" in one of the three positions. Two positions are left (e.g [_, 1, _])
    // We insert "2" in one of the two positions. One position are left (e.g [2, 1, _])
    // We insert "3" in the final position. Permutation is complete (e.g [2, 1, 3])
    // With this logic, each permutation can be represented as a number in a mixed radix system.
    // [1, 2, 3] be represented as (0, 0, 0) or 0
    // [2, 1, 3] be represented as (1, 0, 0) or 1
    // [3, 2, 1] be represented as (2, 1, 0) or 5

    /// Vector passed in defines the sequence of tokens that all permutations will be built from.
    /// 
    /// This vector will be set as the "0th" permutations.
    /// 
    /// E.g passing in the vector [1,2,3] will focus on its permutations (i.e [2,1,3], [3,1,2], etc)
    pub fn new(sequence: Vec<usize>) -> PermutationMapper {
        let bases: Vec<usize> = (1..sequence.len()+1).rev().collect();
        let obj = PermutationMapper{
            core_sequence: sequence,
            mixed_radix_sys: MixedRadix::new(bases), 
        };
        return obj;
    }

    /// Reads a value and maps it to a distinct permutation.
    /// Passing in 0 will output the same sequence given at instantiation.
    pub fn value_to_perm(&self, value: &usize) -> Vec<usize> {
        let mut output_perm: Vec<usize> = vec![0; self.core_sequence.len()];
        // Convert the value to a useful MixedRadix number
        let repr = self.mixed_radix_sys.encode_value(value);
        for (pos, token) in self.core_sequence.iter().enumerate() {
            let shift = repr[pos];
            let mut ind: usize = 0;

            // Use each "digit" in the mixedradix representation to know how much to shift along
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
    /// Reads a permutation and maps it to a distinct value.
    /// Can be thought of as the inverse of value_to_perm.
    /// 
    /// The value resulting from this method should be able to be passed
    /// back into value_to_perm to get back the original permutation.
    /// 
    /// A return of None indicates that the permutation passed in is not a
    /// valid permutation of the sequence given at instantiation.
    fn perm_to_value(&self, permutation: &Vec<usize>) -> Option<usize> {
        // if the lengths of core_sequence and permutation doesn't match then
        // mapping to a value is obviously impossible
        if permutation.len() != self.core_sequence.len() {
            return None;
        }
        
        let mut repr: MixedRadixRepr = Vec::with_capacity(self.core_sequence.len());
        let mut pos_is_filled: Vec<bool> = vec![false; self.core_sequence.len()];
        let max_ind = self.core_sequence.len();

        for token in self.core_sequence.iter() {
            let mut shift = 0;
            let mut ind = 0;
            // move index to first unfilled position
            while ind < max_ind && pos_is_filled[ind] == true {
                ind += 1;
            }
            // keep shifting index
            while ind < max_ind && *token != permutation[ind] {
                // keep track of shifts
                ind += 1;
                shift += 1;
                // autoskip over filled positions
                while ind < max_ind && pos_is_filled[ind] == true {
                    ind += 1;
                }
            }
            // if the index rolls off the "edge" whilst looking for a token match
            // then the permutation passed initially passed in is impossible to map to
            if ind >= max_ind {
                return None;
            }
            // keep track which position has been filled
            pos_is_filled[ind] = true;
            // store the number of shifts
            repr.push(shift);
        }
        return Some(self.mixed_radix_sys.decode_representation(&repr));
    }

    /// Returns a vector of values in which if they were passed into value_to_perm,
    /// the resulting permutation would match the perm_target.
    /// 
    /// The permutation target can be shorter than the sequence passed in at instantiation.
    /// If this is the case, this method will look for permutations whose starting elements matches
    /// the perm_target.
    pub fn possible_values_for(&self, perm_target: &Vec<usize>) -> Vec<usize> {
        let n = perm_target.len();
        // Check for empty perm_target, this means all permutations "fit" the target
        if n == 0 {
            return (1..self.mixed_radix_sys.max_value).collect();
        }
        
        let mut core_leftover = self.core_sequence.clone();
        core_leftover.retain(|x| !perm_target.contains(x));
        
        // Calculate the "minimum" representation in which its value would map to the perm_target
        let mut temp_perm = perm_target.clone();
        temp_perm.append(&mut core_leftover.clone());
        // if no value can be mapped for temp_perm then no values are possible
        let Some(val) = self.perm_to_value(&temp_perm) else {
            return vec![];
        };
        let min_repr = self.mixed_radix_sys.encode_value(&val);
        
        // Calculate the "maximum" representation in which its value would map to the perm_target
        temp_perm = perm_target.clone();
        temp_perm.append(&mut core_leftover.into_iter().rev().collect());
        // if no value can be mapped for temp_perm then no values are possible
        let Some(val) = self.perm_to_value(&temp_perm) else {
            return vec![];
        };
        let max_repr = self.mixed_radix_sys.encode_value(&val);
        
        // Use max and min representations to get range of possible representations
        let sys = MixedRadix::new(
            max_repr
                .iter()
                .zip(min_repr.clone())
                .map(|(max, min)| max+1-min)
                .collect()
        );
        // Iterate through all the representations that fit between max and min repr and store
        // the associated value. (Keep note of the difference between "representation" and "value")
        let mut values = Vec::with_capacity(sys.max_value);
        for repr in sys.into_iter() {
            let cur_val = min_repr
                    .iter()
                    .zip(repr)
                    .map(|(min, num)| min+num)
                    .collect();
            values.push(self.mixed_radix_sys.decode_representation(&cur_val));
        }

        return values;
    }
}


pub struct Handle;
impl SuperPermHandling for Handle {
    fn check_superperm(sequence: &Vec<usize>, n_tokens: usize) -> bool {
        let mapper = PermutationMapper::new((1..n_tokens+1).collect());

        // Brute force approach
        let mut perm_checklist: Vec<bool> = vec![false; mapper.mixed_radix_sys.max_value];
        // Perform rolling window/slice over potential_super and check if the slice is a permutation
        for slice in sequence.windows(mapper.core_sequence.len()) {
            let values = mapper.possible_values_for(&slice.to_vec());
            // values may be an empty list thus check with a for loop
            for value in values {
                perm_checklist[value] = true;
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

    fn create_superperm(n_tokens: usize) -> Vec<usize> {
        let mapper = PermutationMapper::new((1..n_tokens+1).collect());

        // Set an intial sequence to build the superperm from before starting algo
        let mut superperm: Vec<usize> = (1..mapper.core_sequence.len()+1).collect();
        let mut perm_checklist: Vec<bool> = vec![false; mapper.mixed_radix_sys.max_value];
        perm_checklist[0] = true;

        // Loop for all possible permutations to be covered
        for _ in 1..mapper.mixed_radix_sys.max_value {
            let mut trail_matched = false;

            // Loop to grab the trailing sequences of superperm
            for i in (1..mapper.core_sequence.len()).rev() {
                let mut perm_matched = false;
                // Grab the trailing sequence
                let trailing = &superperm[superperm.len()-i..].to_vec();
                // Check if trailing equals the start of any perms left to be checked off
                for value in mapper.possible_values_for(&trailing) {
                    if perm_checklist[value] == false { // Perm has not been checked off
                        // Check off perm and append rest of it onto superperm
                        perm_checklist[value] = true;
                        superperm.extend_from_slice(&mapper.value_to_perm(&value)[i..]);
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

            if !trail_matched {
                // No trailing can be used to build off of. We are free to append on an entire permutation onto the super
                for (i, checked) in perm_checklist.iter().enumerate() {
                    if *checked == false {
                        let mut perm = mapper.value_to_perm(&i);
                        superperm.append(&mut perm);
                        break
                    }
                }
            }
        }
        return superperm;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixedradix_iteration() {
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
    fn mixedradix_edgecase() {
        let system = MixedRadix::new(vec![1,2]);
        let expected = vec![
            vec![0,0],
            vec![0,1],
        ];
        for (i, repr) in system.into_iter().enumerate() {
            assert_eq!(repr, expected[i]);
        }
    }

    #[test]
    fn encode_and_decode_permutations() {
        let helper = PermutationMapper::new((1..6).collect());

        for i in 1..helper.mixed_radix_sys.max_value {
            assert_eq!(
                helper.perm_to_value(&helper.value_to_perm(&i)),
                Some(i)
            );
        }
    }

    #[test]
    fn impossible_permutations() {
        let helper = PermutationMapper::new((1..6).collect());
        let impossible = vec![
            vec![1,2,3,4,4],
            vec![0,2,3,4,5],
            vec![1,2,3,2,5],
            vec![1,2,3,5],
            vec![1,2,3,4,5,6],
        ];
        for imp in impossible {
            println!("{:?}", imp);
            assert_eq!(helper.perm_to_value(&imp), None);
        }
    }
}

