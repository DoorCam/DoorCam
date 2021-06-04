use super::door::*;
use super::user::*;
use super::user_auth::*;
use crate::template_contexts::{AdminNav, AdminViewContext, MainViewContext, Message};
use crate::utils::config::CONFIG;
use crate::utils::guards::{AdminGuard, OnlyUserGuard, UserGuard};
use rocket::request::{FlashMessage, FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket_contrib::templates::{tera::Context, Template};

/// Get the index-view of an user
#[get("/", rank = 1)]
pub fn get_user_index_view(user: OnlyUserGuard, flash: Option<FlashMessage>) -> Template {
    let context = MainViewContext {
        message: flash.map(Message::from),
        cam_url: format!(
            "http://{}:{}/",
            user.user
                .flat
                .map_or_else(String::new, |flat| flat.local_address),
            CONFIG.web.mjpeg_stream_port,
        ),
        activate_door_url: uri!(get_open_door).to_string(),
        change_user_url: uri!(get_change: user.user.id).to_string(),
        logout_url: uri!(get_logout).to_string(),
    };
    Template::render("main_view", &context)
}

#[get("/", rank = 2)]
pub fn redirect_admin_to_index(_admin: AdminGuard) -> Redirect {
    Redirect::to(uri!(get_admin_index_view))
}

/// Get the index-view of an admin
#[get("/admin")]
pub fn get_admin_index_view(_admin: AdminGuard, flash: Option<FlashMessage>) -> Template {
    let context = AdminViewContext {
        message: flash.map(Message::from),
        nav: AdminNav::new(),
    };
    Template::render("admin_view", &context)
}

#[get("/404")]
pub fn get_not_found(_user: UserGuard) -> Template {
    Template::render("404", &Context::new())
}

#[catch(404)]
pub fn not_found_catcher(req: &Request) -> Redirect {
    match UserGuard::from_request(req) {
        Outcome::Failure(_) | Outcome::Forward(_) => Redirect::to(uri!(get_login)),
        Outcome::Success(_) => Redirect::to(uri!(get_not_found)),
    }
}
