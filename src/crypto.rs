use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn fill_rand_array(arr: &mut [u8]) {
    let mut rng = ChaCha20Rng::from_entropy();
    for x in arr {
        *x = rng.gen::<u8>();
    }
}
