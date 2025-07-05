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

    fn compute_value(index: usize, flip: bool, shift: u16) -> u32 {
        if (index % 64) == 0 {
            return 0x0000_0001;
        }

        if !flip {
            1 << shift
        } else {
            0x8000_0000 >> shift
        }
    }

    out.iter_mut().for_each(|element| {
        let shift: u16 = (index as u16) % 32;

        if (index != 0) && (shift == 0) {
            flip = !flip;
        }

        *element = compute_value(index, flip, shift);
        *element = if walking_ones { *element } else { !*element } & u32::MAX;

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
    });

    out
}

/// Create a vector of repeating 0xAAAA and 0x5555.
///
/// # Arguments
///
/// * `length` - Length of the output data pattern vector.
pub async fn make_as5s_data(length: usize) -> Vec<u32> {
    make_xor_data(0xAAAA_AAAA, length).await
}

/// Create a vector of repeating 0x0000 and 0xFFFF.
///
/// # Arguments
///
/// * `length` - Length of the output data pattern vector.
pub async fn make_0sfs_data(length: usize) -> Vec<u32> {
    make_xor_data(0x0000_0000, length).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn walking_1s_test() {
        let result = make_walking_bit_data(37, true).await;
        assert_eq!(
            result,
            vec![
                0x0000_0001,
                0x0000_0002,
                0x0000_0004,
                0x0000_0008,
                0x0000_0010,
                0x0000_0020,
                0x0000_0040,
                0x0000_0080,
                0x0000_0100,
                0x0000_0200,
                0x0000_0400,
                0x0000_0800,
                0x0000_1000,
                0x0000_2000,
                0x0000_4000,
                0x0000_8000,
                0x0001_0000,
                0x0002_0000,
                0x0004_0000,
                0x0008_0000,
                0x0010_0000,
                0x0020_0000,
                0x0040_0000,
                0x0080_0000,
                0x0100_0000,
                0x0200_0000,
                0x0400_0000,
                0x0800_0000,
                0x1000_0000,
                0x2000_0000,
                0x4000_0000,
                0x8000_0000,
                0x8000_0000,
                0x4000_0000,
                0x2000_0000,
                0x1000_0000,
                0x0800_0000,
            ]
        );
    }

    #[tokio::test]
    async fn walking_0s_test() {
        let result = make_walking_bit_data(37, false).await;
        assert_eq!(
            result,
            vec![
                0xFFFF_FFFE,
                0xFFFF_FFFD,
                0xFFFF_FFFB,
                0xFFFF_FFF7,
                0xFFFF_FFEF,
                0xFFFF_FFDF,
                0xFFFF_FFBF,
                0xFFFF_FF7F,
                0xFFFF_FEFF,
                0xFFFF_FDFF,
                0xFFFF_FBFF,
                0xFFFF_F7FF,
                0xFFFF_EFFF,
                0xFFFF_DFFF,
                0xFFFF_BFFF,
                0xFFFF_7FFF,
                0xFFFE_FFFF,
                0xFFFD_FFFF,
                0xFFFB_FFFF,
                0xFFF7_FFFF,
                0xFFEF_FFFF,
                0xFFDF_FFFF,
                0xFFBF_FFFF,
                0xFF7F_FFFF,
                0xFEFF_FFFF,
                0xFDFF_FFFF,
                0xFBFF_FFFF,
                0xF7FF_FFFF,
                0xEFFF_FFFF,
                0xDFFF_FFFF,
                0xBFFF_FFFF,
                0x7FFF_FFFF,
                0x7FFF_FFFF,
                0xBFFF_FFFF,
                0xDFFF_FFFF,
                0xEFFF_FFFF,
                0xF7FF_FFFF,
            ]
        );
    }

    #[tokio::test]
    async fn xor_test1() {
        let result = make_xor_data(0xBEEF_0FF0, 5).await;
        assert_eq!(
            result,
            vec![
                0xBEEF_0FF0,
                0x4110_F00F,
                0xBEEF_0FF0,
                0x4110_F00F,
                0xBEEF_0FF0
            ]
        );
    }

    #[tokio::test]
    async fn as5s_test() {
        let result = make_as5s_data(5).await;
        assert_eq!(
            result,
            vec![
                0xAAAA_AAAA,
                0x5555_5555,
                0xAAAA_AAAA,
                0x5555_5555,
                0xAAAA_AAAA
            ]
        );
    }

    #[tokio::test]
    async fn zeroes_fs_test() {
        let result = make_0sfs_data(5).await;
        assert_eq!(
            result,
            vec![
                0x0000_0000,
                0xFFFF_FFFF,
                0x0000_0000,
                0xFFFF_FFFF,
                0x0000_0000
            ]
        );
    }
}
