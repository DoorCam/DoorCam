//! Are used for the authentification and authorization.

use super::{config::CONFIG, crypto};
use crate::db_entry::{rusqlite, DbConn, UserEntry};
use blake2::{Blake2b, Digest};
use passwords::{analyzer, scorer};
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[cfg(test)]
#[path = "./guards_test.rs"]
mod guards_test;

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

/// A guard which allows all authentificated users.
pub struct UserGuard {
    pub user: UserEntry,
}

impl UserGuard {
    pub fn is_user(&self) -> bool {
        self.user.user_type.is_user()
    }

    pub fn is_admin(&self) -> bool {
        self.user.user_type.is_admin()
    }
}

impl UserGuard {
    /// Checks whether the password is secure or errors if it is weak
    pub fn check_password(pw: &str) -> Result<(), Error> {
        if scorer::score(&analyzer::analyze(pw)) < CONFIG.security.minimal_password_strength {
            return Err(Error::WeakPassword);
        }
        Ok(())
    }
    /// Checks whether the given credentials are valid and writes the user-cookie
    #[allow(clippy::ptr_arg)]
    pub fn authenticate(
        conn: &DbConn,
        cookies: Cookies,
        name: &String,
        pw: &str,
    ) -> Result<UserEntry, Error> {
        // Get UserEntry
        let user = UserEntry::get_active_by_name(&conn, &name)?.ok_or_else(|| {
            crypto::pseudo_hash();
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

        Self::create_user_session(&user, cookies)?;

        Ok(user)
    }
    /// Writes an encrypted cookie with the serialized user data.
    fn create_user_session(
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
    pub fn destroy_user_session(mut cookies: Cookies) {
        cookies.remove_private(Cookie::named("user"));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = Error;

    /// Checks for valid user-cookie in a request
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        return request
            .cookies()
            .get_private("user")
            .map_or(Outcome::Forward(()), |cookie| {
                match serde_json::from_str(cookie.value()) {
                    Ok(user) => Outcome::Success(Self { user }),
                    Err(e) => Outcome::Failure((Status::BadRequest, Error::from(e))),
                }
            });
    }
}

/// A guard which allows only users.
pub struct OnlyUserGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for OnlyUserGuard {
    type Error = Error;

    /// Checks if a valid client is a user
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user_guard = request.guard::<UserGuard>()?;

        if user_guard.is_user() {
            Outcome::Success(Self {
                user: user_guard.user,
            })
        } else {
            Outcome::Forward(())
        }
    }
}

/// A guard which allows only administrators.
pub struct AdminGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = Error;

    /// Checks if a valid client is an admin
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user_guard = request.guard::<UserGuard>()?;

        if user_guard.is_admin() {
            Outcome::Success(Self {
                user: user_guard.user,
            })
        } else {
            Outcome::Forward(())
        }
    }
}
