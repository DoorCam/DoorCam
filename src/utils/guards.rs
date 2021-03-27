//! Are used for the authorization.

use super::auth_manager;
use crate::db_entry::UserEntry;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[cfg(test)]
#[path = "./guards_test.rs"]
mod guards_test;

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

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = auth_manager::Error;

    /// Checks for valid user-cookie in a request
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        return request
            .cookies()
            .get_private("user")
            .map_or(Outcome::Forward(()), |cookie| {
                match serde_json::from_str(cookie.value()) {
                    Ok(user) => Outcome::Success(Self { user }),
                    Err(e) => Outcome::Failure((Status::BadRequest, auth_manager::Error::from(e))),
                }
            });
    }
}

/// A guard which allows only users.
pub struct OnlyUserGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for OnlyUserGuard {
    type Error = auth_manager::Error;

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
    type Error = auth_manager::Error;

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
