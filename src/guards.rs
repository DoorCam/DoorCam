use crate::crypto;
use crate::db_entry::{rusqlite, DbConn, HashEntry, UserEntry};
use blake2::{Blake2b, Digest};
use passwords::{analyzer, scorer};
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    DbError(rusqlite::Error),
    SerializationError(serde_json::error::Error),
    NoUser,
    WrongPassword,
    UnknownHashConfig,
    WeakPassword,
}

impl From<rusqlite::Error> for AuthError {
    fn from(err: rusqlite::Error) -> AuthError {
        AuthError::DbError(err)
    }
}

impl From<serde_json::error::Error> for AuthError {
    fn from(err: serde_json::error::Error) -> AuthError {
        AuthError::SerializationError(err)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::DbError(ref err) => err.fmt(f),
            AuthError::SerializationError(ref err) => err.fmt(f),
            AuthError::NoUser => write!(f, "No user found with this name"),
            AuthError::WrongPassword => write!(f, "The password is wrong"),
            AuthError::UnknownHashConfig => write!(f, "The hash-config is unknown"),
            AuthError::WeakPassword => write!(f, "The password is to weak"),
        }
    }
}

pub struct GuardManager {}

impl GuardManager {
    /// Checks whether the password is secure or errors if it is weak
    pub fn check_password(pw: &str) -> Result<(), AuthError> {
        if scorer::score(&analyzer::analyze(pw)) < 80f64 {
            return Err(AuthError::WeakPassword);
        }
        Ok(())
    }

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

    pub fn auth(
        conn: &DbConn,
        cookies: Cookies,
        name: &String,
        pw: &str,
    ) -> Result<UserEntry, AuthError> {
        let user = UserEntry::get_active_by_name(&conn, &name)?.pop();
        let user = match user {
            Some(user) => user,
            None => return Err(AuthError::NoUser),
        };

        let pw_hash = match user.pw_hash.config.as_str() {
            "plain" => user.pw_hash.hash.clone(),
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
            return Err(AuthError::WrongPassword);
        }

        GuardManager::write_user_cookie(&user, cookies)?;

        Ok(user)
    }

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

    pub fn destroy_user_cookie(mut cookies: Cookies) {
        cookies.remove_private(Cookie::named("user"));
    }
}

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

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserGuard, AuthError> {
        return request
            .cookies()
            .get_private("user")
            .map_or(Outcome::Forward(()), |cookie| {
                match serde_json::from_str(cookie.value()) {
                    Ok(user) => Outcome::Success(UserGuard { user }),
                    Err(e) => Outcome::Failure((Status::BadRequest, AuthError::from(e))),
                }
            });
    }
}

pub struct OnlyUserGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for OnlyUserGuard {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<OnlyUserGuard, AuthError> {
        let user_guard = request.guard::<UserGuard>()?;

        if user_guard.is_user() {
            Outcome::Success(OnlyUserGuard {
                user: user_guard.user,
            })
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct AdminGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminGuard, AuthError> {
        let user_guard = request.guard::<UserGuard>()?;

        if user_guard.is_admin() {
            Outcome::Success(AdminGuard {
                user: user_guard.user,
            })
        } else {
            Outcome::Forward(())
        }
    }
}
