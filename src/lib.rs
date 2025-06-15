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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn walking_1s_test() {
        let result = make_walking_bit_data(10, true);
        assert_eq!(
            result,
            vec![0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0200]
        );
    }

    #[test]
    fn walking_0s_test() {
        let result = make_walking_bit_data(10, false);
        assert_eq!(
            result,
            vec![0xFFFE, 0xFFFD, 0xFFFB, 0xFFF7, 0xFFEF, 0xFFDF, 0xFFBF, 0xFF7F, 0xFEFF, 0xFDFF]
        );
    }
}
