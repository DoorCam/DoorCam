/// Is used for the authentification.
use super::crypto;
use crate::db_entry::{rusqlite, DbConn, HashEntry, UserEntry};
use blake2::{Blake2b, Digest};
use passwords::{analyzer, scorer};
use rocket::http::{Cookie, Cookies};

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    DbError(#[from] rusqlite::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::error::Error),
    #[error("The credentials are invalid")]
    InvalidCredentials,
    #[error("The hash-config is unknown")]
    UnknownHashConfig,
    #[error("The password is to weak")]
    WeakPassword,
}

/// Used for user creation and authentification
pub struct AuthManager {}

impl AuthManager {
    /// Checks whether the password is secure or errors if it is weak
    pub fn check_password(pw: &str) -> Result<(), AuthError> {
        if scorer::score(&analyzer::analyze(pw)) < 80f64 {
            return Err(AuthError::WeakPassword);
        }
        Ok(())
    }

    /// Creates a HashEntry out of a password with a random salt and the currently most secure Hashing algorithm
    pub fn hash(pw: &str) -> HashEntry {
        let mut pw_salt: [u8; 16] = [0; 16];

        crypto::fill_rand_array(&mut pw_salt);
        let pw_salt = base64::encode(pw_salt);

        let pw_hash = base64::encode(
            Blake2b::new()
                .chain(pw)
                .chain(b"$")
                .chain(pw_salt.clone())
                .finalize(),
        );

        HashEntry {
            hash: pw_hash,
            salt: pw_salt,
            config: "Blake2b".to_string(),
        }
    }

    /// Checks whether the given credentials are valid and writes the user-cookie
    #[allow(clippy::ptr_arg)]
    pub fn auth(
        conn: &DbConn,
        cookies: Cookies,
        name: &String,
        pw: &str,
    ) -> Result<UserEntry, AuthError> {
        // Get UserEntry
        let user = UserEntry::get_active_by_name(&conn, &name)?.ok_or_else(|| {
            Blake2b::new().finalize();
            AuthError::InvalidCredentials
        })?;

        // Create hash with matching config
        let pw_hash = match user.pw_hash.config.as_str() {
            "plain" => pw.to_string(),
            "Blake2b" => base64::encode(
                Blake2b::new()
                    .chain(pw)
                    .chain(b"$")
                    .chain(user.pw_hash.salt.clone())
                    .finalize(),
            ),
            _ => return Err(AuthError::UnknownHashConfig),
        };

        if user.pw_hash.hash != pw_hash {
            return Err(AuthError::InvalidCredentials);
        }

        Self::write_user_cookie(&user, cookies)?;

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
}
