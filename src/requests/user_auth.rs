use super::index_view::*;
use crate::db_entry::DbConn;
use crate::template_contexts::{LoginContext, Message};
use crate::utils::auth_manager;
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
    let user = auth_manager::auth(&conn, cookies, &user_data.name, &user_data.pw)
        .map_err(|e| Flash::error(Redirect::to(uri!(get_login)), e.to_string()))?;

    // Redirects to the user-type based main-site
    return Ok(Redirect::to(if user.user_type.is_admin() {
        uri!(get_admin_index_view)
    } else {
        uri!(get_user_index_view)
    }));
}

/// Get logout to destroy the user-cookie
#[get("/logout")]
pub fn get_logout(cookies: Cookies) -> Flash<Redirect> {
    auth_manager::destroy_user_cookie(cookies);
    return Flash::success(
        Redirect::to(uri!(get_login)),
        "Sie wurden erfolgreich ausgeloggt",
    );
}
