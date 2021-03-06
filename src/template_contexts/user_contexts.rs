use super::{AdminNav, Message};
use crate::db_entry::{FlatEntry, UserEntry, UserType};
use crate::requests::user::*;
use serde::Serialize;

#[cfg(test)]
#[path = "./user_contexts_test.rs"]
mod user_contexts_test;

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub message: Option<Message>,
    pub nav: AdminNav,
    pub create_user_url: String,
    pub users: Option<Vec<UserEntry>>,
}

impl UserOverviewContext {
    pub fn view(users: Vec<UserEntry>, message: Option<Message>) -> Self {
        Self {
            message,
            nav: AdminNav::new(),
            create_user_url: uri!(get_create).to_string(),
            users: Some(users),
        }
    }

    pub fn error(message: Message) -> Self {
        Self {
            message: Some(message),
            nav: AdminNav::new(),
            create_user_url: uri!(get_create).to_string(),
            users: None,
        }
    }
}
#[derive(Serialize)]
pub struct UserDetailsContext {
    pub message: Option<Message>,
    pub nav: Option<AdminNav>,
    pub title: String,
    pub is_admin: bool,
    pub user: Option<UserEntry>,
    pub types: Vec<(u16, String)>,
    pub flats: Vec<FlatEntry>,
}

impl UserDetailsContext {
    pub fn error(error: Message) -> Self {
        Self {
            message: Some(error),
            nav: Some(AdminNav::new()),
            title: String::new(),
            is_admin: false,
            user: None,
            types: UserType::get_list(),
            flats: Vec::new(),
        }
    }

    pub fn create(error: Option<Message>, flats: Vec<FlatEntry>) -> Self {
        Self {
            message: error,
            nav: Some(AdminNav::new()),
            title: "Create".to_string(),
            is_admin: true,
            user: None,
            types: UserType::get_list(),
            flats,
        }
    }

    pub fn change(
        error: Option<Message>,
        is_admin: bool,
        user: UserEntry,
        flats: Vec<FlatEntry>,
    ) -> Self {
        Self {
            message: error,
            nav: is_admin.then(AdminNav::new),
            title: "Change".to_string(),
            is_admin,
            user: Some(user),
            types: UserType::get_list(),
            flats,
        }
    }
}
