//! NIST 800-22 tests

use crate::Error;

/// NIST Frequency (Monobit) Test over an iterator of N bits
///
/// See [BitIter](crate::helpers::BitIter) for use with buffers
pub fn nist_freq_monobit(data: impl Iterator<Item = bool>) -> Result<f32, Error> {
    let mut v = 0isize;
    let mut n = 0usize;

    // Sum 0/1 as -1/+1
    for d in data {
        n += 1;

        match d {
            true => v += 1,
            false => v -= 1,
        }
    }

    // Check sample size meets minimum requirements
    if n < 100 {
        return Err(Error::InsufficientSampleSize(n));
    }

    // Compute test statistic
    let s = v.abs() as f32 / libm::sqrtf(n as f32);

    // Compute P-value
    let p = libm::erfcf(s / libm::sqrtf(2.0));

    // Check P value limit. The inverted logic ensures NaNs cause an error.
    if !(p >= 0.01) {
        return Err(Error::BadPValue(p));
    }

    Ok(p)
}

/// NIST Block Frequency Test over an iterator of N bits with block_len sized blocks
///
/// See [BitIter](crate::helpers::BitIter) for use with buffers
pub fn nist_freq_block(
    mut data: impl Iterator<Item = bool>,
    block_len: usize,
) -> Result<f32, Error> {
    let mut num_blocks = 0;
    let mut x2_partial = 0.0;

    // Compute stats for each block
    loop {
        let block = (&mut data).take(block_len);
        let mut block_n = 0;
        let mut block_v = 0;

        // Count the ones in the block
        for v in block {
            block_n += 1;

            if v {
                block_v += 1;
            }
        }

        // Discard if block_n < block_len
        if block_n < block_len {
            break;
        }

        // Compute proportion of ones
        let block_p = (block_v as f32 / block_n as f32) - 0.5;

        let block_x2 = libm::powf(block_p, 2.0);

        // Add to partial x^2 calculation
        x2_partial += block_x2;

        // Update block and value counts
        num_blocks += 1;
    }

    // Compute x^2
    let x2 = 4f32 * block_len as f32 * x2_partial;

    // Compute p
    let p = 1.0 - nist_igamma(num_blocks as f32 / 2.0, x2 / 2.0);

    // Check p value. The inverted logic ensures NaNs cause an error.
    if !(p >= 0.01) {
        return Err(Error::BadPValue(p));
    }

    Ok(p)
}

/// Incomplete gamma function
fn nist_igamma(a: f32, x: f32) -> f32 {
    use special::Gamma;
    x.inc_gamma(a)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use bitvec::prelude::*;
    use rand::{rngs::OsRng, RngCore};

    use super::*;
    use crate::helpers::BitIter;

    #[test]
    fn nist_monobit_ok() {
        let mut rng = OsRng {};
        let mut buff = [0u8; 100];
        rng.fill_bytes(&mut buff);

        nist_freq_monobit(BitIter::new(&buff)).expect("Monobit test failed");
    }

    #[test]
    fn nist_monobit_spec() {
        // 100-bit test buffer
        let buff = bits![
            1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0,
            1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1,
            0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0
        ];

        let p = nist_freq_monobit(buff.iter().by_vals()).expect("Monobit test failed");

        // Check p value matches test vector
        assert_approx_eq!(p, 0.109599);
    }

    #[test]
    fn nist_monobit_fail() {
        nist_freq_monobit(BitIter::from([0xffu8; 128])).expect_err("Monobit p > threshold");
        nist_freq_monobit(BitIter::from([0x00u8; 128])).expect_err("Monobit p > threshold");
    }

    #[test]
    fn nist_block_ok() {
        let mut rng = OsRng {};
        let mut buff = [0u8; 100];
        rng.fill_bytes(&mut buff);

        nist_freq_block(BitIter::new(&buff), 10).expect("Monobit test failed");
    }

    #[test]
    fn nist_block_ex() {
        // Example from specification
        let buff = [0b01100110, 0b00000010];
        let data = BitIter::new(&buff).take(10);

        let p = nist_freq_block(data, 3).expect("Block frequency test failed");

        // Check p value matches test vector
        assert_approx_eq!(p, 0.801252);
    }

    #[test]
    fn nist_block_spec() {
        // 100-bit test from specification
        let buff = bits![
            1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0,
            1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1,
            0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0
        ];

        let p = nist_freq_block(buff.iter().by_vals(), 10).expect("Block frequency test failed");

        // Check p value matches test vector
        assert_approx_eq!(p, 0.706438);
    }

    #[test]
    fn nist_block_fail() {
        // 100-bit test from specification
        let buff = bits![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        nist_freq_block(buff.iter().by_vals(), 10).expect_err("Block frequency test failed");
    }

    #[test]
    fn igamma() {
        let tests = &[
            (1.0, 1.0, 0.6321205588),
            (1.0, 2.0, 0.8646647167),
            // TODO: expand igamma impl to handle > 1 values
            //(1.5, 0.5, 0.1761358672),
            //(10.0, 15.0, 337531.5036053981834998)
        ];

        for (a, x, g) in tests {
            let v = nist_igamma(*a, *x);

            assert_approx_eq!(v, *g, 1e-6f32);
        }
    }
}
