use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// Fills an array with a cryptographic secure random value
pub fn fill_rand_array<T>(arr: &mut [T])
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let mut rng = ChaCha20Rng::from_entropy();
    for x in arr {
        *x = rng.gen::<T>();
    }
}
