use crate::db_entry::UserEntry;
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
    pub error: Option<Message>,
    pub cam_url: String,
    pub activate_door_url: String,
    pub change_user_url: String,
    pub logout_url: String,
}

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct LoginContext {
    pub error: Option<Message>,
}

#[derive(Serialize)]
pub struct UserDetailsContext {
    pub error: Option<Message>,
    pub title: String,
    pub is_admin: bool,
    pub user: Option<UserEntry>,
}

impl UserDetailsContext {
    pub fn error(error: Message) -> UserDetailsContext {
        return UserDetailsContext {
            error: Some(error),
            title: String::new(),
            is_admin: false,
            user: None,
        };
    }

    pub fn create(error: Option<Message>) -> UserDetailsContext {
        return UserDetailsContext {
            error: error,
            title: "Register".to_string(),
            is_admin: true,
            user: None,
        };
    }

    pub fn change(error: Option<Message>, is_admin: bool, user: UserEntry) -> UserDetailsContext {
        return UserDetailsContext {
            error: error,
            title: "Change".to_string(),
            is_admin: is_admin,
            user: Some(user),
        };
    }
}

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub error: Option<Message>,
    pub create_user_url: String,
    pub logout_url: String,
    pub users: Option<Vec<UserEntry>>,
}
