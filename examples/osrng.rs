use rand::{RngCore, rngs::OsRng};
use rngcheck::{nist::*, helpers::*};

fn main() {
    let mut rng = OsRng;

    // Fetch random bytes for tests
    let mut a = [0xFF; 100];
    rng.fill_bytes(&mut a);

    // Check we filled -something- before attempting more in-depth tests
    if &a[..2] == &[0xFF; 2] && &a[a.len() - 2..] == &[0xFF; 2] {
        panic!("RNG seems to have no-op'ed");
    }

    // Run NIST frequency checks
    println!("Monobit result: {:?}", nist_freq_monobit(BitIter::new(&a)));
    println!("Freq block result: {:?}", nist_freq_block(BitIter::new(&a), 10));
}
