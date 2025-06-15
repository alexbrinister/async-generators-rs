//! Simple add function
///
/// Adds two numbers together and produces the sum.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Create a vector of walking bits.
///
/// # Arguments
///
/// * `length` - Length of the output data pattern vector.
/// * `walking_ones` - makes output a walking 1s pattern instead of the default walking 0s.
pub fn make_walking_bit_data(length: usize, walking_ones: bool) -> Vec<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
