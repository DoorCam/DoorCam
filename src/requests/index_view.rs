use super::door::*;
use super::user::*;
use super::user_auth::*;
use crate::template_contexts::{AdminNav, AdminViewContext, MainViewContext, Message, NoContext};
use crate::utils::guards::{AdminGuard, OnlyUserGuard, UserGuard};
use rocket::request::{FlashMessage, FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket_contrib::templates::Template;

/// Get the index-view of an user
#[get("/")]
pub fn get_user_index_view(user: OnlyUserGuard, flash: Option<FlashMessage>) -> Template {
    let context = MainViewContext {
        message: flash.map(Message::from),
        cam_url: "http://doorcam.fritz.box:8081/".to_string(),
        activate_door_url: uri!(get_open_door).to_string(),
        change_user_url: uri!(get_change: user.user.id).to_string(),
        logout_url: uri!(get_logout).to_string(),
    };
    Template::render("main_view", &context)
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
    let context = NoContext {};
    Template::render("404", &context)
}

#[catch(404)]
pub fn not_found_catcher(req: &Request) -> Redirect {
    match UserGuard::from_request(req) {
        Outcome::Failure(_) | Outcome::Forward(_) => Redirect::to(uri!(get_login)),
        Outcome::Success(_) => Redirect::to(uri!(get_not_found)),
    }
}
