use super::{AdminNav, Message};
use crate::db_entry::{FlatEntry, UserEntry, UserType};
use crate::requests::user::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub message: Option<Message>,
    pub nav: AdminNav,
    pub create_user_url: String,
    pub users: Option<Vec<UserEntry>>,
}

impl UserOverviewContext {
    pub fn view(users: Vec<UserEntry>) -> Self {
        UserOverviewContext {
            message: None,
            nav: AdminNav::new(),
            create_user_url: uri!(get_create).to_string(),
            users: Some(users),
        }
    }

    pub fn error(message: Message) -> Self {
        UserOverviewContext {
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
        return UserDetailsContext {
            message: Some(error),
            nav: Some(AdminNav::new()),
            title: String::new(),
            is_admin: false,
            user: None,
            types: UserType::get_list(),
            flats: Vec::new(),
        };
    }

    pub fn create(error: Option<Message>, flats: Vec<FlatEntry>) -> Self {
        return UserDetailsContext {
            message: error,
            nav: Some(AdminNav::new()),
            title: "Register".to_string(),
            is_admin: true,
            user: None,
            types: UserType::get_list(),
            flats: flats,
        };
    }

    pub fn change(
        error: Option<Message>,
        is_admin: bool,
        user: UserEntry,
        flats: Vec<FlatEntry>,
    ) -> Self {
        return UserDetailsContext {
            message: error,
            nav: if is_admin {
                Some(AdminNav::new())
            } else {
                None
            },
            title: "Change".to_string(),
            is_admin: is_admin,
            user: Some(user),
            types: UserType::get_list(),
            flats: flats,
        };
    }
}
