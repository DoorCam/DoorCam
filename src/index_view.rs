use crate::guards::{OnlyUserGuard, UserGuard};
use crate::template_contexts::{MainViewContext, NoContext};
use crate::user_auth::*;
use crate::user_requests::*;
use rocket::request::{FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(user: OnlyUserGuard) -> Template {
    let context = MainViewContext {
        cam_url: "http://doorcam.fritz.box:8081/".to_string(),
        change_user_url: uri!(get_change: user.user.id).to_string(),
        logout_url: uri!(get_logout).to_string(),
    };
    Template::render("main_view", &context)
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
