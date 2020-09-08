use crate::db_conn::DbConn;
use crate::error_request::*;
use crate::guards::GuardManager;
use crate::index_view::*;
use crate::template_contexts::NoContext;
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
}

#[get("/login")]
pub fn get_login() -> Template {
    let context = NoContext {};
    Template::render("user_login", &context)
}

#[post("/login", data = "<user_data>")]
pub fn post_login_data(user_data: Form<UserForm>, conn: DbConn, cookies: Cookies) -> Redirect {
    let _user = match GuardManager::auth(conn, cookies, &user_data.name, &user_data.pw) {
        Err(e) => return Redirect::to(uri!(error: e.to_string())),
        Ok(user) => user,
    };
    return Redirect::to(uri!(index));
}
