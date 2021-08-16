//! Cryptographic helper/wrapper function(s).
use super::config::CONFIG;
use aes::Aes128;
use argon2::password_hash::{
    Error, Ident, Output, ParamsString, PasswordHash, PasswordHasher, Salt, SaltString,
};
use argon2::{Argon2, Version};
use block_modes::block_padding::Iso7816;
use block_modes::{BlockMode, BlockModeError, Pcbc};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::convert::{TryFrom, TryInto};

#[cfg(test)]
#[path = "./crypto_test.rs"]
mod crypto_test;

#[derive(Clone, Debug)]
pub struct PlaintextParams {
    pub padding_character: u8,
}

impl PlaintextParams {
    pub const DEFAULT_PADDING_CHARACTER: u8 = b'\0';
}

impl Default for PlaintextParams {
    fn default() -> Self {
        Self {
            padding_character: Self::DEFAULT_PADDING_CHARACTER,
        }
    }
}

impl<'a> TryFrom<&'a PasswordHash<'a>> for PlaintextParams {
    type Error = Error;

    fn try_from(value: &'a PasswordHash<'a>) -> Result<Self, Self::Error> {
        let mut params = Self::default();

        if let Some(character) = value.params.get_str("pad") {
            if let Some(character) = character.chars().next() {
                params.padding_character = character as u8;
            }
        }
        Ok(params)
    }
}

impl TryFrom<PlaintextParams> for ParamsString {
    type Error = Error;

    fn try_from(params: PlaintextParams) -> Result<Self, Self::Error> {
        let mut output = Self::new();
        output.add_str("pad", params.padding_character.to_string().as_str())?;
        Ok(output)
    }
}

pub struct Plaintext;

impl Plaintext {
    pub const IDENT: Ident<'static> = Ident::new("plain");
    pub const DEFAULT_SALT: &'static str = "salt";
}

impl PasswordHasher for Plaintext {
    type Params = PlaintextParams;
    fn hash_password<'a>(
        &self,
        password: &[u8],
        _algorithm: Option<Ident<'a>>,
        params: Self::Params,
        _salt: impl Into<Salt<'a>>,
    ) -> Result<PasswordHash<'a>, Error> {
        let mut hash: Vec<_> = std::iter::repeat(params.padding_character)
            .take(10_usize.saturating_sub(password.len()))
            .collect();
        hash.extend_from_slice(password);

        Ok(PasswordHash {
            algorithm: Self::IDENT,
            version: None,
            params: params.try_into()?,
            salt: Some(Salt::new(Self::DEFAULT_SALT)?),
            hash: Some(Output::new(hash.as_slice())?),
        })
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
