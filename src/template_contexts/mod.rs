use crate::user_entry::UserEntry;
use rocket::request::FlashMessage;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub category: String,
    pub content: String,
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
pub struct UserCreateContext {
    pub error: Option<Message>,
}

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub error: Option<Message>,
    pub users: Option<Vec<UserEntry>>,
}
