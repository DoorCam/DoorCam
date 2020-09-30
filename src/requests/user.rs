use crate::db_entry::{DbConn, FlatEntry, UserEntry, UserType};
use crate::guards::{AdminGuard, UserGuard};
use crate::template_contexts::{Message, UserDetailsContext, UserOverviewContext};
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
    pw_repeat: String,
    user_type: Option<UserType>,
    active: Option<bool>,
    flat_id: Option<u32>,
}

#[get("/admin/user/create")]
pub fn get_create(_admin: AdminGuard, conn: DbConn, flash: Option<FlashMessage>) -> Template {
    let context = match FlatEntry::get_all(&conn) {
        Err(e) => UserDetailsContext::error(Message::error(e.to_string())),
        Ok(flats) => UserDetailsContext::create(flash.map(Message::from), flats),
    };
    Template::render("user_details", &context)
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
    if let Err(e) = UserEntry::create(
        &conn,
        &user_data.name,
        &user_data.pw,
        user_data.user_type.unwrap_or(UserType::User),
        user_data.active.unwrap_or(false),
        user_data.flat_id,
    ) {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            format!("DB Error: {}", e),
        ));
    }

    return Ok(Redirect::to(uri!(get_users)));
}

#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, conn: DbConn) -> Template {
    let context = match UserEntry::get_all(&conn) {
        Ok(users) => UserOverviewContext::view(users),
        Err(e) => UserOverviewContext::error(Message::error(format!("DB Error: {}", e))),
    };
    Template::render("user_overview", &context)
}

#[delete("/admin/user/delete/<id>")]
pub fn delete(admin: AdminGuard, conn: DbConn, id: u32) -> Flash<()> {
    if admin.user.id == id {
        return Flash::error((), "Can't delete yourself");
    }
    if let Err(e) = UserEntry::delete(&conn, id) {
        return Flash::error((), e.to_string());
    };

    Flash::success((), "User deleted")
}

#[get("/admin/user/change/<id>")]
pub fn get_change(
    user_guard: UserGuard,
    conn: DbConn,
    flash: Option<FlashMessage>,
    id: u32,
) -> Result<Template, Status> {
    if !user_guard.user.user_type.is_admin() && user_guard.user.id != id {
        return Err(Status::Forbidden);
    }
    let flats = match FlatEntry::get_all(&conn) {
        Err(e) => {
            return Ok(Template::render(
                "user_details",
                &UserDetailsContext::error(Message::error(e.to_string())),
            ))
        }
        Ok(flats) => flats,
    };
    let context = match UserEntry::get_by_id(&conn, id).as_mut() {
        Ok(users) => match users.pop() {
            Some(user) => UserDetailsContext::change(
                flash.map(Message::from),
                user_guard.user.user_type.is_admin(),
                user,
                flats,
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
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    if user_guard.is_user() && user_guard.user.id != id {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Forbidden",
        ));
    }
    if !user_guard.user.user_type.is_admin()
        && (user_data.user_type.is_some()
            || user_data.active.is_some()
            || user_data.flat_id.is_some())
    {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Don't manipulate the user-type or active-Flag",
        ));
    }
    if user_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Name is empty",
        ));
    }
    if !user_data.pw.is_empty() && user_data.pw != user_data.pw_repeat {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Passwords are not the same",
        ));
    }
    if let Err(e) = UserEntry::change(
        &conn,
        id,
        &user_data.name,
        &user_data.pw,
        user_data.user_type.unwrap_or(user_guard.user.user_type),
        if user_guard.is_user() {
            true
        } else {
            user_data.active.unwrap_or(false)
        },
        if user_guard.is_user() {
            user_guard.user.flat_id
        } else {
            user_data.flat_id
        },
    ) {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            format!("DB Error: {}", e),
        ));
    }

    return Ok(Redirect::to(uri!(get_users)));
}
