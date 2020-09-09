use crate::guards::{OnlyUserGuard, UserGuard};
use crate::template_contexts::{CamContext, NoContext};
use crate::user_auth::*;
use rocket::request::{FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(_user: OnlyUserGuard) -> Template {
    let context = CamContext {
        cam_url: "http://doorcam.fritz.box:8081/".to_string(),
    };
    Template::render("cam", &context)
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
