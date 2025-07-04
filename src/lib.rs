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
pub async fn make_walking_bit_data(length: usize, walking_ones: bool) -> Vec<u32> {
    let mut out: Vec<u32> = vec![0; length];
    let mut flip = false;
    let mut index: usize = 0;

    fn compute_value(index: usize, flip: bool, shift: u16) -> u16 {
        if (index % 32) == 0 {
            return 0x0001;
        }

        if !flip {
            1 << shift
        } else {
            0x8000 >> shift
        }
    }

    out.iter_mut().for_each(|element| {
        let shift: u16 = (index as u16) % 16;

        if (index != 0) && (shift == 0) {
            flip = !flip;
        }

        *element = compute_value(index, flip, shift) as u32;
        *element = if walking_ones { *element } else { !*element } & (u16::MAX as u32);

        index += 1;
    });

    out
}

/// Create a vector of repeating words of a seed value XOR'd with 0 then u32::MAX.
///
/// # Arguments
///
/// * `seed`   - the seed value to begin the pattern.
/// * `length` - Length of the output data pattern vector.
pub async fn make_xor_data(seed: u32, length: usize) -> Vec<u32> {
    let mut out: Vec<u32> = vec![0; length];
    let mut fizz = true;

    out.iter_mut().for_each(|element| {
        *element = seed ^ (if fizz { 0x0000 } else { u32::MAX });
        fizz = !fizz;
        *element &= 0x0000FFFF;
    });

    out
}

/// Create a vector of repeating 0xAAAA and 0x5555.
///
/// # Arguments
///
/// * `length` - Length of the output data pattern vector.
pub async fn make_as5s_data(length: usize) -> Vec<u32> {
    make_xor_data(0xAAAAAAAA, length).await
}

/// Create a vector of repeating 0x0000 and 0xFFFF.
///
/// # Arguments
///
/// * `length` - Length of the output data pattern vector.
pub async fn make_0sfs_data(length: usize) -> Vec<u32> {
    make_xor_data(0x00000000, length).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn walking_1s_test() {
        let result = make_walking_bit_data(10, true).await;
        assert_eq!(
            result,
            vec![0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0200]
        );
    }

    #[tokio::test]
    async fn walking_0s_test() {
        let result = make_walking_bit_data(10, false).await;
        assert_eq!(
            result,
            vec![0xFFFE, 0xFFFD, 0xFFFB, 0xFFF7, 0xFFEF, 0xFFDF, 0xFFBF, 0xFF7F, 0xFEFF, 0xFDFF]
        );
    }

    #[tokio::test]
    async fn xor_test1() {
        let result = make_xor_data(0x0FF0, 5).await;
        assert_eq!(result, vec![0x0FF0, 0xF00F, 0x0FF0, 0xF00F, 0x0FF0]);
    }

    #[tokio::test]
    async fn as5s_test() {
        let result = make_as5s_data(5).await;
        assert_eq!(result, vec![0xAAAA, 0x5555, 0xAAAA, 0x5555, 0xAAAA]);
    }

    #[tokio::test]
    async fn zeroes_fs_test() {
        let result = make_0sfs_data(5).await;
        assert_eq!(result, vec![0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000]);
    }
}
