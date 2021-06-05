//! Cryptographic helper/wrapper function(s).
use super::config::CONFIG;
use crate::db_entry::HashEntry;
use aes::Aes128;
use blake2::{Blake2b, Digest};
use block_modes::block_padding::Iso7816;
use block_modes::{BlockMode, BlockModeError, Pcbc};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[cfg(test)]
#[path = "./crypto_test.rs"]
mod crypto_test;

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
pub fn hash(pw: &str) -> HashEntry {
    let mut pw_salt: [u8; 16] = [0; 16];

    fill_rand_array(&mut pw_salt);

    let pw_hash = base64::encode(
        Blake2b::new()
            .chain(pw)
            .chain(b"$")
            .chain(pw_salt)
            .chain(b"$")
            .chain(&CONFIG.security.hash_pepper)
            .finalize(),
    );
    let encoded_pw_salt = base64::encode(pw_salt);

    HashEntry {
        hash: pw_hash,
        salt: encoded_pw_salt,
        config: "Blake2b".to_string(),
    }
}

pub fn pseudo_hash() {
    Blake2b::new().finalize();
}

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
