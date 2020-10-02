use super::index_view::*;
use crate::db_entry::DbConn;
use crate::guards::GuardManager;
use crate::template_contexts::{LoginContext, Message};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct LoginForm {
    name: String,
    pw: String,
}

#[get("/login")]
pub fn get_login(flash: Option<FlashMessage>) -> Template {
    let context = LoginContext {
        message: flash.map(Message::from),
    };
    Template::render("login", &context)
}

#[post("/login", data = "<user_data>")]
pub fn post_login_data(
    user_data: Form<LoginForm>,
    conn: DbConn,
    cookies: Cookies,
) -> Result<Redirect, Flash<Redirect>> {
    let user = match GuardManager::auth(&conn, cookies, &user_data.name, &user_data.pw) {
        Err(e) => return Err(Flash::error(Redirect::to(uri!(get_login)), e.to_string())),
        Ok(user) => user,
    };
    return Ok(Redirect::to(if user.user_type.is_admin() {
        uri!(get_admin_index_view)
    } else {
        uri!(get_user_index_view)
    }));
}

#[get("/logout")]
pub fn get_logout(cookies: Cookies) -> Flash<Redirect> {
    GuardManager::destroy_user_cookie(cookies);
    return Flash::success(
        Redirect::to(uri!(get_login)),
        "Sie wurden erfolgreich ausgeloggt",
    );
}