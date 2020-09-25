use crate::db_entry::UserEntry;
use crate::requests::{user::*, user_auth::*};
use rocket::request::FlashMessage;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub category: String,
    pub content: String,
}

impl Message {
    pub fn error(msg: String) -> Message {
        return Message {
            category: "error".to_string(),
            content: msg,
        };
    }
}

impl From<FlashMessage<'_, '_>> for Message {
    fn from(msg: FlashMessage) -> Message {
        return Message {
            category: msg.name().to_string().clone(),
            content: msg.msg().to_string().clone(),
        };
    }
}

#[derive(Serialize)]
pub struct MainViewContext {
    pub message: Option<Message>,
    pub cam_url: String,
    pub activate_door_url: String,
    pub change_user_url: String,
    pub logout_url: String,
}

#[derive(Serialize)]
pub struct AdminViewContext {
    pub message: Option<Message>,
    pub nav: AdminNav,
}

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct LoginContext {
    pub message: Option<Message>,
}

#[derive(Serialize)]
pub struct AdminNav {
    user_overview_url: String,
    logout_url: String,
}

impl AdminNav {
    pub fn new() -> Self {
        AdminNav {
            user_overview_url: uri!(get_users).to_string(),
            logout_url: uri!(get_logout).to_string(),
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
}

impl UserDetailsContext {
    pub fn error(error: Message) -> UserDetailsContext {
        return UserDetailsContext {
            message: Some(error),
            nav: Some(AdminNav::new()),
            title: String::new(),
            is_admin: false,
            user: None,
        };
    }

    pub fn create(error: Option<Message>) -> UserDetailsContext {
        return UserDetailsContext {
            message: error,
            nav: Some(AdminNav::new()),
            title: "Register".to_string(),
            is_admin: true,
            user: None,
        };
    }

    pub fn change(error: Option<Message>, is_admin: bool, user: UserEntry) -> UserDetailsContext {
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
        };
    }
}

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
