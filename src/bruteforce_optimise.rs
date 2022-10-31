#![allow(dead_code)]
// Same goals and fundamental algorithims as bruteforce_no_optimise
// except optimisations have been made using better data structuring.

use crate::base::*;

/// Class to encode any value into a different base where
/// each "position" can be of a different base
pub struct VariableBaseSystem {
    pub bases: Vec<usize>,
    pub max_value: usize,
}

/// A type alias over a Vec<usize>.
/// 
/// Short for "VariableBaseRepresentation".
type VarBaseRepr = Vec<usize>;

impl VariableBaseSystem {
    /// bases parameter details:
    ///  - Vec passed in expected to be the base for each position,
    ///  - Values in the front of vec refer to least significant positions.
    ///  - It also indirectly sets the number of positions available.
    pub fn new(bases: Vec<usize>) -> VariableBaseSystem{
        // Calculate maximum representable value with the bases passed in
        let mut max: usize = 1;
        for base in &bases {
            max *= base;
        }
        return VariableBaseSystem {
            bases: bases,
            max_value: max,
        }
    }

    /// Interprets a value into a representation of the bases specified at instantiation
    /// 
    /// E.g The value 54 encoded to the bases (5, 4, 3) will be represented as (4, 2, 2)
    pub fn encode_value(&self, val: &usize) -> VarBaseRepr {
        let mut representation: VarBaseRepr = vec![0; self.bases.len()];
        let mut carry_over = val.clone();
        for (i, base) in self.bases.iter().enumerate() {
            representation[i] = carry_over % base;
            carry_over = carry_over / base;
        }
        return representation;
    }
    /// Interprets a representation into a value. Inverse of encode_value
    pub fn decode_representation(&self, repr: &VarBaseRepr) -> usize {
        let mut sum: usize = 0;
        let mut position_mult: usize = 1;
        for (pos, base) in self.bases.iter().enumerate() {
            sum += repr[pos] * position_mult;
            position_mult *= base;
        }
        return sum;
    }
}

/// Class to help with the handling of permutations for a sequence of the form: (1,2,3,..,n)
pub struct PermutationsHelper {
    pub sequence_n: usize,
    pub core_sequence: Vec<usize>,
    pub base_encoder: VariableBaseSystem,
}
impl PermutationsHelper {
    // Why use VariableBaseSystem for permutations?
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
            base_encoder: VariableBaseSystem::new(bases), 
        };
        return obj;
    }

    /// Convert a VariableBaseRepresentation into a distinct permutation
    pub fn encoded_to_perm(&self, encoded: &VarBaseRepr) -> Vec<usize> {
        let mut output_perm: Vec<usize> = vec![0; self.sequence_n];
        for (pos, element) in self.core_sequence.iter().enumerate() {
            let mut shift = encoded[pos];
            let mut target: usize = 0;
            // Use each number in the encoded Vec to know how much to shift along
            // before inserting. Positions already filled in output_perm are auto-skipped.
            while shift > 0 || output_perm[target] != 0 {
                if output_perm[target] != 0 {
                    target += 1;
                }
                else {
                    target += 1;
                    shift -= 1;
                }
            }
            output_perm[target] = element.clone();
        }
        return output_perm
    }

    /// Returns a vector of encoded representations for permutations whose 
    /// starting sequence matches perm_target.
    pub fn possible_encodes(&self, perm_target: &Vec<usize>) -> Vec<VarBaseRepr> {
        let filler = self.sequence_n+1;
        let mut working_space: Vec<usize> = vec![filler; self.sequence_n];
        let mut encoded_pos_possibilities: Vec<Vec<usize>> = vec![Vec::new(); self.sequence_n];

        // Loop through each number in the core_sequence
        for (encoded_pos, n) in self.core_sequence.iter().enumerate() {
            let mut i: usize = 0;
            let mut shifted: usize = 0;
            if perm_target.contains(&n) { // Number is part of the perm_target
                // Set i to first free position in working_space
                while working_space[i] != filler {
                    i += 1;
                }
                // Increase i until perm_target[i] == n
                while perm_target[i] != *n {
                    // Auto skip over filled positions
                    if working_space[i] != filler {
                        i += 1;
                    }
                    // Track how many unfilled positions are skipped over
                    else {
                        i += 1;
                        shifted += 1;
                    }
                }
                // At this point, perm_target[i] == n
                working_space[i] = n.clone(); // Fill in working_space[i] to match perm_target
                // Since n is part of perm_target, we only need to the shifted value for this encoded position
                encoded_pos_possibilities[encoded_pos].push(shifted);
            }
            else { // Number is not part of the perm_target
                // Set i to the first free position in working_space
                while working_space[i] != filler {
                    i += 1;
                }
                // Keep shifting until we are beyond the length of perm_target
                while i < perm_target.len() {
                    // Auto skip over filled positions
                    if working_space[i] != filler {
                        i += 1;
                    }
                    else { // Track how many unfilled positions are skipped
                        i += 1;
                        shifted += 1;
                    }
                }
                let position_base = self.sequence_n-n+1;
                if shifted >= position_base { // Check if perm_target is possible to match
                    // For an impossible perm_target, shifted will exceed the maximum possibilities for
                    // this position (indicated by position_base)
                    return Vec::new();
                }
                else { // perm_target is possible
                    // All values >=shifted are valid for this encoded position
                    encoded_pos_possibilities[encoded_pos] = (shifted..self.sequence_n-n+1).collect();
                }
            }
        }

        // Start combining the numbers at each encoded position into a list of encoded representations
        let mut bases: Vec<usize> = Vec::with_capacity(self.sequence_n);
        for possibilites in &encoded_pos_possibilities {
            bases.push(possibilites.len());
        }
        let indexer = VariableBaseSystem::new(bases);
        let mut repr_possibilities: Vec<VarBaseRepr> = Vec::with_capacity(indexer.max_value);

        // Using VariableBaseSystem to iterate through all combinations of encoded_pos_possibilities
        // and store each combination into repr_possiblities.
        for value in 0..indexer.max_value {
            let curr_indexes = indexer.encode_value(&value);
            let mut repr: VarBaseRepr = Vec::new();
            for i in 0..curr_indexes.len() {
                repr.push(encoded_pos_possibilities[i][curr_indexes[i]].clone());
            }
            repr_possibilities.push(repr);
        }
        return repr_possibilities;
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
            // Loop to grab the trailing sequences of superperm
            for i in (0..self.sequence_n).rev() {
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
                    break;
                }
                // If loop reaches this point, trailing sequence didn't match the start of any unchecked
                // perms. Next loop round, the size of the trailing sequence gets smaller.
                // When i = 0, the trailing sequence will be empty []. In this case the first unchecked
                // permutation will get appended onto the superperm fully
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


