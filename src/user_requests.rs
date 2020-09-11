use crate::db_conn::DbConn;
use crate::guards::AdminGuard;
use crate::template_contexts::{Message, UserCreateContext, UserOverviewContext};
use crate::user_entry::UserEntry;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
    pw_repeat: String,
    admin: bool,
}

#[get("/admin/user/create")]
pub fn get_create(_admin: AdminGuard, flash: Option<FlashMessage>) -> Template {
    let context = UserCreateContext {
        error: flash.map(|msg| Message::from(msg)),
    };
    Template::render("user_create", &context)
}

#[post("/admin/user/create", data = "<user_data>")]
pub fn post_create_data(
    user_data: Form<UserForm>,
    _admin: AdminGuard,
    conn: DbConn,
) -> Result<Redirect, Flash<Redirect>> {
    if user_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            "Name is empty",
        ));
    }
    if user_data.pw.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            "Password is empty".to_string(),
        ));
    }
    if user_data.pw != user_data.pw_repeat {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            "Passwords are not the same",
        ));
    }
    match UserEntry::create(conn, &user_data.name, &user_data.pw, user_data.admin) {
        Err(e) => {
            return Err(Flash::error(
                Redirect::to(uri!(get_create)),
                format!("DB Error: {}", e),
            ))
        }
        _ => {}
    }

    return Ok(Redirect::to(uri!(get_users)));
}

#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, conn: DbConn) -> Template {
    let context = match UserEntry::get_all(conn) {
        Ok(users) => UserOverviewContext {
            users: Some(users),
            error: None,
        },
        Err(e) => UserOverviewContext {
            error: Some(Message {
                category: "error".to_string(),
                content: format!("DB Error: {}", e),
            }),
            users: None,
        },
    };
    Template::render("user_overview", &context)
}

#[delete("/admin/user/delete/<id>")]
pub fn delete_user(conn: DbConn, id: u32) -> Result<(), String> {
    return UserEntry::delete(conn, id).map_err(|e| e.to_string());
}
