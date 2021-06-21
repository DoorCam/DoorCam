use super::index_view::*;
use super::{ErrorIntoFlash, ResultFlash};
use crate::db_entry::DbConn;
use crate::template_contexts::{LoginContext, Message};
use crate::utils::guards::UserGuard;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

/// Struct with all login form data.
#[derive(FromForm)]
pub struct LoginForm {
    name: String,
    pw: String,
}

/// Get the login form
#[get("/login")]
pub fn get_login(flash: Option<FlashMessage>) -> Template {
    let context = LoginContext {
        message: flash.map(Message::from),
    };
    Template::render("login", &context)
}

/// Post the user-data to do the login logic
#[post("/login", data = "<user_data>")]
pub fn post_login_data(
    user_data: Form<LoginForm>,
    conn: DbConn,
    cookies: Cookies,
) -> Result<Redirect, Flash<Redirect>> {
    UserGuard::authenticate(&conn, cookies, &user_data.name, &user_data.pw)
        .map_err(|e| e.into_redirect_flash(uri!(get_login)))?;

    Ok(Redirect::to(uri!(get_user_index_view)))
}

/// Get logout to destroy the user-cookie
#[get("/logout")]
pub fn get_logout(user_guard: UserGuard, conn: DbConn, cookies: Cookies) -> ResultFlash<Redirect> {
    user_guard
        .destroy_user_session(&conn, cookies)
        .map_err(|e| e.into_redirect_flash(uri!(get_user_index_view)))?;

    Ok(Flash::success(
        Redirect::to(uri!(get_login)),
        "Sie wurden erfolgreich ausgeloggt",
    ))
}
