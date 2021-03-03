use super::rusqlite::{
    self,
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
};
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[cfg(test)]
#[path = "./user_type_test.rs"]
mod user_type_test;

/// A logical enum of the user_type database field.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum UserType {
    User,
    Admin,
}

impl UserType {
    pub fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    /// get a Vector of touples of the value and string of all types
    pub fn get_list() -> Vec<(u16, String)> {
        vec![Self::User.into(), Self::Admin.into()]
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::User => write!(f, "User"),
            Self::Admin => write!(f, "Admin"),
        }
    }
}

impl TryFrom<u16> for UserType {
    type Error = String;

    fn try_from(num: u16) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(Self::User),
            2 => Ok(Self::Admin),
            _ => Err(format!("Unknown user-type: {}", num)),
        }
    }
}

impl From<UserType> for u16 {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::User => 1,
            UserType::Admin => 2,
        }
    }
}

impl From<UserType> for (u16, String) {
    fn from(user_type: UserType) -> Self {
        (user_type.into(), user_type.to_string())
    }
}

/// needed to convert from the raw SQL-value
impl FromSql for UserType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match Self::try_from(u16::column_result(value)?) {
            Ok(user_type) => Ok(user_type),
            Err(_) => Err(FromSqlError::OutOfRange(value.as_i64()?)),
        }
    }
}

/// needed to convert to the raw SQL-value
impl ToSql for UserType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let num: u16 = self.clone().into();
        Ok(ToSqlOutput::from(num))
    }
}

/// needed to convert from the raw Form-value
impl<'v> FromFormValue<'v> for UserType {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        match Self::try_from(u16::from_form_value(form_value)?) {
            Ok(user_type) => Ok(user_type),
            Err(_) => Err(form_value),
        }
    }
}
