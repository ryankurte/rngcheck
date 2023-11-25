/// Helper for bit-wise iteration through slices
pub struct BitIter<B: AsRef<[u8]>> {
    buff: B,
    i: usize,
    j: usize,
}

impl<B: AsRef<[u8]>> BitIter<B> {
    /// Create a new [BitIter] over the provided buffer
    pub fn new(buff: B) -> Self {
        Self { buff, i: 0, j: 0 }
    }
}

impl<B: AsRef<[u8]>> From<B> for BitIter<B> {
    /// Convert a type `B: AsRef<[u8]>` into a `BitIter<B>`
    fn from(value: B) -> Self {
        BitIter::new(value)
    }
}

/// Bit-wise [Iterator] implementation for [BitIter]
impl<B: AsRef<[u8]>> Iterator for BitIter<B> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.buff.as_ref();

        // Check for remaining data
        if self.i >= d.len() {
            return None;
        }

        // Fetch current bit value
        let v = d[self.i] & (1 << self.j) != 0;

        // Increment index
        if self.j < 7 {
            self.j += 1;
        } else {
            self.i += 1;
            self.j = 0;
        }

        // Return value
        Some(v)
    }
}

/// Helper for bit-wise iteration from an RNG
///
/// This pulls random data out of the RNG in chunks of 32 bits, and produces them one by one for
/// testing.
pub struct BitsFromRng<'a, R: rand_core_0_6::RngCore> {
    rng: &'a mut R,
    remaining: usize,
    buffer: u32,
    buffered: u8,
}

impl<'a, R: rand_core_0_6::RngCore> BitsFromRng<'a, R> {
    pub fn new(rng: &'a mut R, items: usize) -> Self {
        Self {
            rng,
            remaining: items,
            buffer: 0,
            buffered: 0,
        }
    }
}
impl<'a, R: rand_core_0_6::RngCore> Iterator for BitsFromRng<'a, R> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;

        if self.buffered == 0 {
            self.buffer = self.rng.next_u32();
            self.buffered = 32;
        }
        let result = self.buffer & 1 != 0;
        self.buffer >>= 1;
        self.buffered -= 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn bit_iter() {
        let tests = &[(
            &[0b0000_0001, 0b0000_0010, 0b0100_0000, 0b1000_0000],
            &[
                true, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false, false, false, false, false, false, false, true, false,
                false, false, false, false, false, false, false, true,
            ],
        )];

        for (buff, bits) in tests {
            let i = BitIter::new(&buff[..]);
            let v: Vec<bool> = i.collect();
            assert_eq!(&v, bits);
        }
    }

    #[test]
    fn from_rng() {
        let bits: Vec<_> = BitsFromRng::new(&mut rand::rngs::OsRng, 123).collect();
        assert_eq!(bits.len(), 123);
    }
}
