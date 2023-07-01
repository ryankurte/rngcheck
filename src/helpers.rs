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
}
