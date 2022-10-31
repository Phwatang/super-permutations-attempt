/// Declare the common functionality for each implementation
pub trait SuperPermHandling {
    /// Creates a superpmutation. 
    /// 
    /// Tokens used are the numbers (1,2,3,4, ...,n_tokens).
    fn create_superperm(n_tokens: usize) -> Vec<usize>;

    /// Checks if sequence passed in is a valid superpermutation.
    /// 
    /// n_tokens specifies the amount of unique tokens within the sequence. See
    /// documentation for create_superperm for what the tokens should be.
    fn check_superperm(sequence: &Vec<usize>, n_tokens: usize) -> bool;
}
