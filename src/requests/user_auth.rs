use super::index_view::*;
use super::user::*;
use crate::db_entry::DbConn;
use crate::guards::GuardManager;
use crate::template_contexts::{LoginContext, Message};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
}

#[get("/login")]
pub fn get_login(flash: Option<FlashMessage>) -> Template {
    let context = LoginContext {
        error: flash.map(|msg| Message::from(msg)),
    };
    Template::render("user_login", &context)
}

#[post("/login", data = "<user_data>")]
pub fn post_login_data(
    user_data: Form<UserForm>,
    conn: DbConn,
    cookies: Cookies,
) -> Result<Redirect, Flash<Redirect>> {
    let user = match GuardManager::auth(conn, cookies, &user_data.name, &user_data.pw) {
        Err(e) => return Err(Flash::error(Redirect::to(uri!(get_login)), e.to_string())),
        Ok(user) => user,
    };
    return Ok(Redirect::to(if user.admin {
        uri!(get_users)
    } else {
        uri!(index)
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
