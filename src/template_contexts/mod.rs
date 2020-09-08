use crate::user_entry::UserEntry;
use serde::Serialize;

#[derive(Serialize)]
pub struct CamContext {
    pub cam_url: String,
}

#[derive(Serialize)]
pub struct NoContext {}

#[derive(Serialize)]
pub struct ErrorContext {
    pub error: String,
}

#[derive(Serialize)]
pub struct UserOverviewContext {
    pub users: Vec<UserEntry>,
}
