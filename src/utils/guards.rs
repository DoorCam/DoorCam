//! Are used for the authentification and authorization.

use super::{config::CONFIG, crypto};
use crate::db_entry::{rusqlite, DbConn, Entry, UserEntry, UserSessionEntry};
use bool_ext::BoolExt;
use chrono::offset::Utc;
use passwords::{analyzer, scorer};
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./guards_test.rs"]
mod guards_test;

/// All errors which could happen during user creation, authentification and authorization.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] rusqlite::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::error::Error),
    #[error(transparent)]
    Decode(#[from] base64::DecodeError),
    #[error("The credentials are invalid")]
    InvalidCredentials,
    #[error("The hash-config is unknown")]
    UnknownHashConfig,
    #[error("The hash-config is blocked")]
    BlockedHashConfig,
    #[error("The password is to weak")]
    WeakPassword,
    #[error("There is no database")]
    NoDatabase,
}

/// A guard which allows all authentificated users.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGuard {
    pub user: UserEntry,
    session: UserSessionEntry,
}

impl UserGuard {
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        self.user.user_type.is_user()
    }

    #[inline(always)]
    pub fn is_admin(&self) -> bool {
        self.user.user_type.is_admin()
    }
}

impl UserGuard {
    /// Checks whether the password is secure or errors if it is weak
    #[inline]
    pub fn check_password(pw: &str) -> Result<(), Error> {
        (scorer::score(&analyzer::analyze(pw)) >= CONFIG.security.minimal_password_strength_score)
            .err(Error::WeakPassword)?;

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
        let user = UserEntry::get_active_by_name(conn, name)?.ok_or_else(|| {
            crypto::pseudo_hash();
            Error::InvalidCredentials
        })?;

        todo!();

        Self::create_user_session(conn, user.clone(), cookies)?;

        Ok(user)
    }
    /// Writes an encrypted cookie with the serialized user data.
    fn create_user_session(
        conn: &DbConn,
        user: UserEntry,
        mut cookies: Cookies,
    ) -> Result<(), Error> {
        let session = UserSessionEntry {
            id: (),
            login_datetime: Utc::now(),
            user: user.id,
        }
        .create(conn)?;

        let session_guard = Self { user, session };

        cookies.add_private(
            Cookie::build("user_session_guard", serde_json::to_string(&session_guard)?)
                .permanent()
                .finish(),
        );
        Ok(())
    }
    /// Destroys the private encrypted user cookie.
    pub fn destroy_user_session(
        self,
        conn: &DbConn,
        mut cookies: Cookies,
    ) -> Result<(), rusqlite::Error> {
        self.session.delete(conn)?;

        cookies.remove_private(Cookie::named("user_session_guard"));
        Ok(())
    }

    fn validate(&self, conn: &DbConn) -> Result<bool, rusqlite::Error> {
        let session = UserSessionEntry::get_by_id(conn, self.session.get_id())?;

        Ok(session.map_or(false, |session| {
            session == self.session && session.user.get_id() == self.user.get_id()
        }))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = Error;

    /// Checks for valid user-cookie in a request
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let conn = request
            .guard::<DbConn>()
            .map_failure(|(status, _)| (status, Error::NoDatabase))?;

        return request.cookies().get_private("user_session_guard").map_or(
            Outcome::Forward(()),
            |cookie| match serde_json::from_str::<Self>(cookie.value()) {
                Ok(user_guard) => match user_guard.validate(&conn) {
                    Ok(true) => Outcome::Success(user_guard),
                    Ok(false) => Outcome::Forward(()),
                    Err(e) => Outcome::Failure((Status::BadRequest, Error::from(e))),
                },
                Err(e) => Outcome::Failure((Status::BadRequest, Error::from(e))),
            },
        );
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
