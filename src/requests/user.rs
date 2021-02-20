use crate::db_entry::{DbConn, FlatEntry, UserEntry, UserType};
use crate::template_contexts::{Message, UserDetailsContext, UserOverviewContext};
use crate::utils::guards::{AdminGuard, UserGuard};
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

/// Struct with all user-details form data.
/// The optional values have to be considered as non-admins editing themselves aren't allowed to change these values.
#[derive(FromForm)]
pub struct UserForm {
    name: String,
    pw: String,
    pw_repeat: String,
    user_type: Option<UserType>,
    active: Option<bool>,
    flat_id: Option<u32>,
}

/// Get the form to create an user
#[get("/admin/user/create")]
pub fn get_create(_admin: AdminGuard, conn: DbConn, flash: Option<FlashMessage>) -> Template {
    let context = match FlatEntry::get_all(&conn) {
        Err(e) => UserDetailsContext::error(Message::error(e.to_string())),
        Ok(flats) => UserDetailsContext::create(flash.map(Message::from), flats),
    };
    Template::render("user_details", &context)
}

/// Post the user-data in order to create an user
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

/// Shows all users
#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, conn: DbConn) -> Template {
    let context = match UserEntry::get_all(&conn) {
        Ok(users) => UserOverviewContext::view(users),
        Err(e) => UserOverviewContext::error(Message::error(format!("DB Error: {}", e))),
    };
    Template::render("user_overview", &context)
}

/// Deletes an user
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

/// Get the form to modify an user
#[get("/admin/user/change/<id>")]
pub fn get_change(
    user_guard: UserGuard,
    conn: DbConn,
    flash: Option<FlashMessage>,
    id: u32,
) -> Result<Template, Status> {
    // An ordinary user is only allowed to modify himself
    if !user_guard.user.user_type.is_admin() && user_guard.user.id != id {
        return Err(Status::Forbidden);
    }

    // Get all FlatEntrys to display them in a select-box
    let flats = match FlatEntry::get_all(&conn) {
        Err(e) => {
            return Ok(Template::render(
                "user_details",
                &UserDetailsContext::error(Message::error(e.to_string())),
            ))
        }
        Ok(flats) => flats,
    };

    // Get the UserEntry in order to know the old values
    let context = match UserEntry::get_by_id(&conn, id) {
        Ok(Some(user)) => UserDetailsContext::change(
            flash.map(Message::from),
            user_guard.user.user_type.is_admin(),
            user,
            flats,
        ),
        Ok(None) => UserDetailsContext::error(Message::error("No user found".to_string())),
        Err(e) => UserDetailsContext::error(Message::error(e.to_string())),
    };
    Ok(Template::render("user_details", &context))
}

/// Post user data to modify the user
#[post("/admin/user/change/<id>", data = "<user_data>")]
pub fn post_change_data(
    user_guard: UserGuard,
    conn: DbConn,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    // An ordinary user is only allowed to modify himself
    if user_guard.is_user() && user_guard.user.id != id {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Forbidden",
        ));
    }

    // A non-admin isn't allowed to change these fields
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

    // If the password is updated, the two fields must be the same
    if !user_data.pw.is_empty() && user_data.pw != user_data.pw_repeat {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Passwords are not the same",
        ));
    }

    // Unwraps all optional fields with its user-type based default value
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
            user_guard.user.flat.map(|flat| flat.id)
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
