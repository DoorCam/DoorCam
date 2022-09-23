//! Cryptographic helper/wrapper function(s).
use super::config::{PasswordHashConfig, CONFIG};
use argon2::{password_hash, Argon2};
use easy_ext::ext;
use either::Either;
use password_hash::{PasswordHasher, SaltString};
use pcbc::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::convert::TryInto;

mod plaintext;

pub use plaintext::Plaintext;

#[cfg(test)]
mod mod_test;

#[ext(DefaultWithSecret)]
pub impl<'key> Argon2<'key> {
    fn default_with_secret(secret: &'key [u8]) -> Self {
        Self::new_with_secret(
            secret,
            argon2::Algorithm::default(),
            argon2::Version::default(),
            argon2::Params::default(),
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
pub fn hash(password: &str) -> Result<String, Either<argon2::Error, password_hash::Error>> {
    if password.is_empty() {
        return Ok(String::new());
    }

    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let hasher = default_hasher().map_err(Either::Left)?;

    hasher
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(Either::Right)
}

fn default_hasher() -> Result<Argon2<'static>, argon2::Error> {
    match &CONFIG.security.used_password_hash {
        PasswordHashConfig::Argon2(argon2) => {
            let (algorithm, params) = argon2.try_into()?;
            Argon2::new_with_secret(
                &CONFIG.security.hash_pepper,
                algorithm,
                argon2::Version::default(),
                params,
            )
        }
    }
}

/// This should sleep as long (+- random duration) as `hash` would take. This is needed to prevent a malicious actor from enumerating the valid users.
pub fn pseudo_hash() {
    let hashing_duration = rand::rngs::OsRng.gen_range(
        CONFIG.security.minimal_hashing_duration..=CONFIG.security.maximal_hashing_duration,
    );

    std::thread::sleep(hashing_duration);
}

type Aes128PcbcEnc = pcbc::Encryptor<aes::Aes128>;
type Aes128PcbcDec = pcbc::Decryptor<aes::Aes128>;

pub fn symetric_encrypt(key: &[u8; 16], init_vec: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes128PcbcEnc::new_from_slices(key, init_vec).unwrap();
    cipher.encrypt_padded_vec_mut::<Pkcs7>(plaintext)
}

pub fn symetric_decrypt(
    key: &[u8; 16],
    init_vec: &[u8; 16],
    ciphertext: &[u8],
) -> Result<Vec<u8>, UnpadError> {
    let cipher = Aes128PcbcDec::new_from_slices(key, init_vec).unwrap();
    cipher.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
}
