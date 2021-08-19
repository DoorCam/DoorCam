//! Cryptographic helper/wrapper function(s).
use super::config::CONFIG;
use aes::Aes128;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::{Argon2, Version};
use block_modes::block_padding::Iso7816;
use block_modes::{BlockMode, BlockModeError, Pcbc};
use easy_ext::ext;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

mod plaintext;

pub use plaintext::Plaintext;

#[cfg(test)]
mod mod_test;

#[ext(DefaultWithSecret)]
pub impl<'key> Argon2<'key> {
    fn default_with_secret(secret: &'key [u8]) -> Self {
        let params = argon2::Params::default();

        Self::new(
            Some(secret),
            params.t_cost,
            params.m_cost,
            params.p_cost,
            params.version,
        )
        .expect("invalid default Argon2 params")
    }
}

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

/// Creates a HashEntry out of a password with a random salt and the currently most secure Hashing algorithm
pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let argon2 = Argon2::new(
        Some(&CONFIG.security.hash_pepper),
        2,
        15_360,
        1,
        Version::default(),
    )
    .unwrap();
    argon2
        .hash_password_simple(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn pseudo_hash() {}

type Aes128Pcbc = Pcbc<Aes128, Iso7816>;

pub fn symetric_encrypt(key: &[u8; 16], init_vec: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes128Pcbc::new_var(key, init_vec).unwrap();
    cipher.encrypt_vec(plaintext)
}

pub fn symetric_decrypt(
    key: &[u8; 16],
    init_vec: &[u8; 16],
    ciphertext: &[u8],
) -> Result<Vec<u8>, BlockModeError> {
    let cipher = Aes128Pcbc::new_var(key, init_vec).unwrap();
    cipher.decrypt_vec(ciphertext)
}
