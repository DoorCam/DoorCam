//! Is used for user creation and authentification.
use super::{config::CONFIG, crypto};
use crate::db_entry::{rusqlite, DbConn, HashEntry, UserEntry};
use blake2::{Blake2b, Digest};
use passwords::{analyzer, scorer};
use rocket::http::{Cookie, Cookies};

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] rusqlite::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::error::Error),
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
    #[error("The credentials are invalid")]
    InvalidCredentials,
    #[error("The hash-config is unknown")]
    UnknownHashConfig,
    #[error("The password is to weak")]
    WeakPassword,
}
/// Checks whether the password is secure or errors if it is weak
pub fn check_password(pw: &str) -> Result<(), Error> {
    if scorer::score(&analyzer::analyze(pw)) < CONFIG.security.minimal_password_strength {
        return Err(Error::WeakPassword);
    }
    Ok(())
}

/// Creates a HashEntry out of a password with a random salt and the currently most secure Hashing algorithm
pub fn hash(pw: &str) -> HashEntry {
    let mut pw_salt: [u8; 16] = [0; 16];

    crypto::fill_rand_array(&mut pw_salt);

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

/// Checks whether the given credentials are valid and writes the user-cookie
#[allow(clippy::ptr_arg)]
pub fn auth(conn: &DbConn, cookies: Cookies, name: &String, pw: &str) -> Result<UserEntry, Error> {
    // Get UserEntry
    let user = UserEntry::get_active_by_name(&conn, &name)?.ok_or_else(|| {
        Blake2b::new().finalize();
        Error::InvalidCredentials
    })?;

    // Create hash with matching config
    let pw_hash = match user.pw_hash.config.as_str() {
        "plain" => pw.to_string(),
        "Blake2b" => {
            let decoded_pw_salt = base64::decode(&user.pw_hash.salt)?;
            base64::encode(
                Blake2b::new()
                    .chain(pw)
                    .chain(b"$")
                    .chain(decoded_pw_salt)
                    .chain(b"$")
                    .chain(&CONFIG.security.hash_pepper)
                    .finalize(),
            )
        }
        _ => return Err(Error::UnknownHashConfig),
    };

    if user.pw_hash.hash != pw_hash {
        return Err(Error::InvalidCredentials);
    }

    write_user_cookie(&user, cookies)?;

    Ok(user)
}

/// Writes an encrypted cookie with the serialized user data.
fn write_user_cookie(
    user: &UserEntry,
    mut cookies: Cookies,
) -> Result<(), serde_json::error::Error> {
    cookies.add_private(
        Cookie::build("user", serde_json::to_string(&user)?)
            .permanent()
            .finish(),
    );
    Ok(())
}

/// Destroys the private encrypted user cookie.
pub fn destroy_user_cookie(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("user"));
}
