use super::{AdminNav, Message};
use crate::db_entry::FlatEntry;
use crate::requests::flat::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct FlatOverviewContext {
    pub message: Option<Message>,
    pub nav: AdminNav,
    pub create_flat_url: String,
    pub flats: Option<Vec<FlatEntry>>,
}

impl FlatOverviewContext {
    pub fn view(flats: Vec<FlatEntry>) -> Self {
        FlatOverviewContext {
            message: None,
            nav: AdminNav::new(),
            create_flat_url: uri!(get_create).to_string(),
            flats: Some(flats),
        }
    }

    pub fn error(message: Message) -> Self {
        FlatOverviewContext {
            message: Some(message),
            nav: AdminNav::new(),
            create_flat_url: uri!(get_create).to_string(),
            flats: None,
        }
    }
}
#[derive(Serialize)]
pub struct FlatDetailsContext {
    pub message: Option<Message>,
    pub nav: AdminNav,
    pub title: String,
    pub flat: Option<FlatEntry>,
}

impl FlatDetailsContext {
    pub fn error(error: Message) -> Self {
        FlatDetailsContext {
            message: Some(error),
            nav: AdminNav::new(),
            title: String::new(),
            flat: None,
        }
    }

    pub fn create(error: Option<Message>) -> Self {
        FlatDetailsContext {
            message: error,
            nav: AdminNav::new(),
            title: "Register".to_string(),
            flat: None,
        }
    }

    pub fn change(error: Option<Message>, flat: FlatEntry) -> Self {
        FlatDetailsContext {
            message: error,
            nav: AdminNav::new(),
            title: "Change".to_string(),
            flat: Some(flat),
        }
    }
}
