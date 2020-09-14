use crate::user_entry::UserEntry;
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
pub struct CamContext {
    pub cam_url: String,
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
    pub transfer_method: String,
    pub is_admin: bool,
    pub user: Option<UserEntry>,
}

impl UserDetailsContext {
    pub fn error(error: Message) -> UserDetailsContext {
        return UserDetailsContext {
            error: Some(error),
            transfer_method: String::new(),
            is_admin: false,
            user: None,
        };
    }

    pub fn create(error: Option<Message>) -> UserDetailsContext {
        return UserDetailsContext {
            error: error,
            transfer_method: "post".to_string(),
            is_admin: true,
            user: None,
        };
    }

    pub fn change(error: Option<Message>, is_admin: bool, user: UserEntry) -> UserDetailsContext {
        return UserDetailsContext {
            error: error,
            transfer_method: "".to_string(),
            is_admin: is_admin,
            user: Some(user),
        };
    }
}

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub error: Option<Message>,
    pub users: Option<Vec<UserEntry>>,
}
