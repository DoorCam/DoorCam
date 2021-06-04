//! Here are all contexts which are needed to generate the HTML from the templates.

use crate::requests::{flat::*, index_view::*, user::*, user_auth::*};
use rocket::request::FlashMessage;
use serde::Serialize;

#[cfg(test)]
#[path = "./template_test.rs"]
mod template_test;

/// Struct which is used to show the user a message
#[derive(Serialize)]
pub struct Message {
    pub category: String,
    pub content: String,
}

impl Message {
    pub fn error(msg: String) -> Self {
        Self {
            category: "error".to_string(),
            content: msg,
        }
    }
}

impl From<FlashMessage<'_, '_>> for Message {
    fn from(msg: FlashMessage) -> Self {
        return Self {
            category: msg.name().to_string(),
            content: msg.msg().to_string(),
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
pub struct LoginContext {
    pub message: Option<Message>,
}

/// A struct which is used in all other admin contextes to show the nav-bar
#[derive(Serialize)]
pub struct AdminNav {
    admin_view_url: String,
    flat_overview_url: String,
    user_overview_url: String,
    logout_url: String,
}

impl AdminNav {
    pub fn new() -> Self {
        Self {
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
