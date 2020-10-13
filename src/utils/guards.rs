/// Are used for the authentification and authorization.
use super::auth_manager::AuthError;
use crate::db_entry::UserEntry;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

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
    type Error = AuthError;

    /// Checks for valid user-cookie in a request
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

/// A guard which allows only users.
pub struct OnlyUserGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for OnlyUserGuard {
    type Error = AuthError;

    /// Checks if a valid client is a user
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

/// A guard which allows only administrators.
pub struct AdminGuard {
    pub user: UserEntry,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = AuthError;

    /// Checks if a valid client is an admin
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
