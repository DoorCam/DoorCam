//! A `PasswordHasher` for plaintext passwords stored in a PHC-string. These should only be used
//! during onboarding and not in a production environment.

use argon2::password_hash::{
    Error, Ident, Output, ParamsString, PasswordHash, PasswordHasher, Salt,
};
use std::convert::{TryFrom, TryInto};

#[cfg(test)]
#[path = "./plaintext_test.rs"]
mod plaintext_test;

/// Plaintext password hash parameters. These are parameters which can be encoded into a PHC hash string.
#[derive(Clone, Debug)]
pub struct PlaintextParams {
    // An ASCII value which is used for padding before the payload to have at least ten bytes of
    // hash. It is ASCII encoded in the PHC string.
    pub padding_character: u8,
}

impl PlaintextParams {
    pub const DEFAULT_PADDING_CHARACTER: u8 = b'\0';

    #[cfg(test)]
    pub fn new(padding_character: u8) -> Self {
        Self { padding_character }
    }
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
            if let Ok(character) = character.parse::<u8>() {
                params.padding_character = character;
            }
        }
        Ok(params)
    }
}

impl TryFrom<PlaintextParams> for ParamsString {
    type Error = Error;

    fn try_from(params: PlaintextParams) -> Result<Self, Self::Error> {
        let mut output = Self::new();
        if params.padding_character != PlaintextParams::DEFAULT_PADDING_CHARACTER {
            output.add_str("pad", params.padding_character.to_string().as_str())?;
        }
        Ok(output)
    }
}

/// Plaintext context.
/// It pads the plaintext password to at least 8 bytes and encodes it as Base64. It uses 'salt' as
/// a salt as it is needed in the PHC format. This version of Base64 doesn't have '=' characters at
/// the end.
/// Some examples:
/// 'admin' -> '$plain$salt$AAAAAABhZG1pbg'
/// 'test' -> '$plain$pad=42$salt$KioqKioqdGVzdA'
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
