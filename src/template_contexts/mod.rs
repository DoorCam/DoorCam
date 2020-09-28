use crate::requests::{flat::*, index_view::*, user::*, user_auth::*};
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
    admin_view_url: String,
    flat_overview_url: String,
    user_overview_url: String,
    logout_url: String,
}

impl AdminNav {
    pub fn new() -> Self {
        AdminNav {
            admin_view_url: uri!(get_admin_index_view).to_string(),
            flat_overview_url: uri!(get_flats).to_string(),
            user_overview_url: uri!(get_users).to_string(),
            logout_url: uri!(get_logout).to_string(),
        }
    }
}

pub mod user_contexts;
pub use user_contexts::{UserDetailsContext, UserOverviewContext};

pub mod flat_contexts;
pub use flat_contexts::{FlatDetailsContext, FlatOverviewContext};
