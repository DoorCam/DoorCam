use super::user_auth::*;
use crate::db_entry::{DbConn, UserEntry};
use crate::guards::{AdminGuard, UserGuard};
use crate::template_contexts::{Message, UserDetailsContext, UserOverviewContext};
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use std::sync::{Arc, Mutex};

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
    pw_repeat: String,
    admin: Option<bool>,
}

#[get("/admin/user/create")]
pub fn get_create(_admin: AdminGuard, flash: Option<FlashMessage>) -> Template {
    let context = UserDetailsContext::create(flash.map(|msg| Message::from(msg)));
    Template::render("user_details", &context)
}

#[post("/admin/user/create", data = "<user_data>")]
pub fn post_create_data(
    user_data: Form<UserForm>,
    _admin: AdminGuard,
    conn: DbConn,
    sync_flag: State<Arc<Mutex<bool>>>,
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
    match UserEntry::create(
        conn,
        &user_data.name,
        &user_data.pw,
        user_data.admin.unwrap_or(false),
    ) {
        Err(e) => {
            return Err(Flash::error(
                Redirect::to(uri!(get_create)),
                format!("DB Error: {}", e),
            ))
        }
        _ => {}
    }

    // sync iot::EventHandler
    match sync_flag.lock() {
        Ok(mut sf) => *sf = true,
        Err(e) => return Err(Flash::error(Redirect::to(uri!(get_create)), e.to_string())),
    };

    return Ok(Redirect::to(uri!(get_users)));
}

#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, conn: DbConn) -> Template {
    let context = match UserEntry::get_all(conn) {
        Ok(users) => UserOverviewContext {
            users: Some(users),
            error: None,
            create_user_url: uri!(get_create).to_string(),
            logout_url: uri!(get_logout).to_string(),
        },
        Err(e) => UserOverviewContext {
            error: Some(Message {
                category: "error".to_string(),
                content: format!("DB Error: {}", e),
            }),
            users: None,
            create_user_url: uri!(get_create).to_string(),
            logout_url: uri!(get_logout).to_string(),
        },
    };
    Template::render("user_overview", &context)
}

#[delete("/admin/user/delete/<id>")]
pub fn delete(
    admin: AdminGuard,
    conn: DbConn,
    sync_flag: State<Arc<Mutex<bool>>>,
    id: u32,
) -> Flash<()> {
    if admin.user.id == id {
        return Flash::error((), "Can't delete yourself");
    }
    if let Err(e) = UserEntry::delete(conn, id) {
        return Flash::error((), e.to_string());
    };

    // sync iot::EventHandler
    match sync_flag.lock() {
        Ok(mut sf) => *sf = true,
        Err(e) => return Flash::error((), e.to_string()),
    };

    return Flash::success((), "User deleted");
}

#[get("/admin/user/change/<id>")]
pub fn get_change(
    user_guard: UserGuard,
    conn: DbConn,
    flash: Option<FlashMessage>,
    id: u32,
) -> Result<Template, Status> {
    if !user_guard.user.admin && user_guard.user.id != id {
        return Err(Status::Forbidden);
    }
    let context = match UserEntry::get_by_id(conn, id).as_mut() {
        Ok(users) => match users.pop() {
            Some(user) => UserDetailsContext::change(
                flash.map(|msg| Message::from(msg)),
                user_guard.user.admin,
                user,
            ),
            None => UserDetailsContext::error(Message::error("No user found".to_string())),
        },
        Err(e) => UserDetailsContext::error(Message::error(e.to_string())),
    };
    Ok(Template::render("user_details", &context))
}

#[post("/admin/user/change/<id>", data = "<user_data>")]
pub fn post_change_data(
    user_guard: UserGuard,
    conn: DbConn,
    sync_flag: State<Arc<Mutex<bool>>>,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    if !user_guard.user.admin && user_guard.user.id != id {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Forbidden",
        ));
    }
    if !user_guard.user.admin && user_data.admin.is_some() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Don't manipulate the admin-Flag",
        ));
    }
    if user_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Name is empty",
        ));
    }
    if !user_data.pw.is_empty() {
        if user_data.pw != user_data.pw_repeat {
            return Err(Flash::error(
                Redirect::to(uri!(get_change: id)),
                "Passwords are not the same",
            ));
        }
    }
    if let Err(e) = UserEntry::change(
        conn,
        id,
        &user_data.name,
        &user_data.pw,
        user_data.admin.unwrap_or(false),
    ) {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            format!("DB Error: {}", e),
        ));
    }

    // sync iot::EventHandler
    match sync_flag.lock() {
        Ok(mut sf) => *sf = true,
        Err(e) => {
            return Err(Flash::error(
                Redirect::to(uri!(get_change: id)),
                e.to_string(),
            ))
        }
    };

    return Ok(Redirect::to(uri!(get_users)));
}
