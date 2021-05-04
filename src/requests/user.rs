use super::{index_view::rocket_uri_macro_get_user_index_view, FormIntoEntry};
use crate::db_entry::{DbConn, Entry, FlatEntry, UserEntry, UserType};
use crate::template_contexts::{Message, UserDetailsContext, UserOverviewContext};
use crate::utils::auth_manager;
use crate::utils::guards::{AdminGuard, OnlyUserGuard, UserGuard};
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

impl FormIntoEntry<UserEntry<(), u32>, UserEntry<u32, u32>> for UserForm {
    fn into_insertable(self) -> UserEntry<(), u32> {
        let hash = auth_manager::hash(&self.pw);

        UserEntry {
            id: (),
            name: self.name,
            pw_hash: hash,
            user_type: self.user_type.unwrap_or(UserType::User),
            active: self.active.unwrap_or(false),
            flat: self.flat_id,
        }
    }

    fn into_entry(self, id: u32) -> UserEntry<u32, u32> {
        let hash = auth_manager::hash(&self.pw);

        UserEntry {
            id,
            name: self.name,
            pw_hash: hash,
            user_type: self.user_type.unwrap_or(UserType::User),
            active: self.active.unwrap_or(false),
            flat: self.flat_id,
        }
    }
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
    auth_manager::check_password(&user_data.pw)
        .map_err(|e| Flash::error(Redirect::to(uri!(get_create)), e.to_string()))?;

    user_data
        .into_inner()
        .into_insertable()
        .create(&conn)
        .map_err(|e| Flash::error(Redirect::to(uri!(get_create)), format!("DB Error: {}", e)))?;

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
    if let Err(e) = UserEntry::<_>::delete_entry(&conn, id) {
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
#[post("/admin/user/change/<id>", data = "<user_data>", rank = 2)]
pub fn admin_post_change_data(
    _user_guard: AdminGuard,
    conn: DbConn,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let changed_password = !user_data.pw.is_empty();

    if user_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Name is empty",
        ));
    }

    // If the password is updated, the two fields must be the same
    if changed_password && user_data.pw != user_data.pw_repeat {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Passwords are not the same",
        ));
    }
    if changed_password {
        auth_manager::check_password(&user_data.pw)
            .map_err(|e| Flash::error(Redirect::to(uri!(get_change: id)), e.to_string()))?;
    }

    let entry = user_data.into_inner().into_entry(id);

    let update_result = match changed_password {
        true => entry.update(&conn),
        false => entry.update_without_password(&conn),
    };

    update_result.map_err(|e| {
        Flash::error(
            Redirect::to(uri!(get_change: id)),
            format!("DB Error: {}", e),
        )
    })?;

    return Ok(Redirect::to(uri!(get_users)));
}

/// Post user data to modify the user
#[post("/admin/user/change/<id>", data = "<user_data>")]
pub fn user_post_change_data(
    user_guard: OnlyUserGuard,
    conn: DbConn,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    // An ordinary user is only allowed to modify himself
    if user_guard.user.id != id {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Forbidden",
        ));
    }

    let changed_password = !user_data.pw.is_empty();

    // A non-admin isn't allowed to change these fields
    if user_data.user_type.is_some() || user_data.active.is_some() || user_data.flat_id.is_some() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Don't manipulate the user-type, active-Flag or flat-ID",
        ));
    }

    if user_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Name is empty",
        ));
    }

    // If the password is updated, the two fields must be the same
    if changed_password && user_data.pw != user_data.pw_repeat {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Passwords are not the same",
        ));
    }
    if changed_password {
        auth_manager::check_password(&user_data.pw)
            .map_err(|e| Flash::error(Redirect::to(uri!(get_change: id)), e.to_string()))?;
    }

    let entry = user_data.into_inner().into_entry(id);

    let update_result = match changed_password {
        true => entry.update_unprivileged(&conn),
        false => entry.update_unprivileged_without_password(&conn),
    };

    update_result.map_err(|e| {
        Flash::error(
            Redirect::to(uri!(get_change: id)),
            format!("DB Error: {}", e),
        )
    })?;

    return Ok(Redirect::to(uri!(get_user_index_view)));
}
