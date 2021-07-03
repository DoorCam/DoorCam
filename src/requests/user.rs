use super::{user_auth::rocket_uri_macro_get_login, ErrorIntoFlash, FormIntoEntry, ResultFlash};
use crate::db_entry::{DbConn, Entry, FlatEntry, UserEntry, UserSessionEntry, UserType};
use crate::template_contexts::{Message, UserDetailsContext, UserOverviewContext};
use crate::utils::crypto;
use crate::utils::guards::{AdminGuard, OnlyUserGuard, UserGuard};
use bool_ext::BoolExt;
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use std::ops::Not;

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
        let hash = crypto::hash(&self.pw);

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
        let hash = crypto::hash(&self.pw);

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
    user_data
        .name
        .is_empty()
        .not()
        .err_with(|| "Name is empty".into_redirect_flash(uri!(get_create)))?;

    user_data
        .pw
        .is_empty()
        .not()
        .err_with(|| "Password is empty".into_redirect_flash(uri!(get_create)))?;

    (user_data.pw == user_data.pw_repeat)
        .err_with(|| "Passwords are not the same".into_redirect_flash(uri!(get_create)))?;

    UserGuard::check_password(&user_data.pw)
        .map_err(|e| e.into_redirect_flash(uri!(get_create)))?;

    user_data
        .into_inner()
        .into_insertable()
        .create(&conn)
        .map_err(|e| e.into_redirect_flash(uri!(get_create)))?;

    return Ok(Redirect::to(uri!(get_users)));
}

/// Shows all users
#[get("/admin/user")]
pub fn get_users(_admin: AdminGuard, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let context = match UserEntry::get_all(&conn) {
        Ok(users) => UserOverviewContext::view(users, flash.map(Message::from)),
        Err(e) => UserOverviewContext::error(Message::error(format!("DB Error: {}", e))),
    };
    Template::render("user_overview", &context)
}

/// Deletes an user
#[delete("/admin/user/delete/<id>")]
pub fn delete(admin: AdminGuard, conn: DbConn, id: u32) -> ResultFlash<()> {
    (admin.user.id != id).err_with(|| "Can't delete yourself".into_flash())?;

    UserSessionEntry::delete_by_user(&conn, id)
        .and_then(|_| UserEntry::<_>::delete_entry(&conn, id))
        .map_err(|e| e.into_flash())?;

    Err(Flash::success((), "User deleted"))
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
    (user_guard.user.user_type.is_admin() || user_guard.user.id == id).err(Status::Forbidden)?;

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
    _admin_guard: AdminGuard,
    conn: DbConn,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let unchanged_password = user_data.pw.is_empty();
    let changed_password = !unchanged_password;

    user_data
        .name
        .is_empty()
        .not()
        .err_with(|| "Name is empty".into_redirect_flash(uri!(get_change: id)))?;

    // If the password is updated, the two fields must be the same
    (unchanged_password || user_data.pw == user_data.pw_repeat)
        .err_with(|| "Passwords are not the same".into_redirect_flash(uri!(get_change: id)))?;

    if changed_password {
        UserGuard::check_password(&user_data.pw)
            .map_err(|e| e.into_redirect_flash(uri!(get_change: id)))?;
    }

    let entry = user_data.into_inner().into_entry(id);

    match changed_password {
        true => entry.update(&conn),
        false => entry.update_without_password(&conn),
    }
    .and_then(|_| UserSessionEntry::delete_by_user(&conn, entry.get_id()))
    .map_err(|e| e.into_redirect_flash(uri!(get_change: id)))?;

    return Ok(Redirect::to(uri!(get_users)));
}

/// Post user data to modify the user
#[post("/admin/user/change/<id>", data = "<user_data>")]
pub fn user_post_change_data(
    user_guard: OnlyUserGuard,
    conn: DbConn,
    id: u32,
    user_data: Form<UserForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    // An ordinary user is only allowed to modify himself
    (user_guard.user.get_id() == id)
        .err_with(|| "Forbidden".into_redirect_flash(uri!(get_change: id)))?;

    let unchanged_password = user_data.pw.is_empty();
    let changed_password = !unchanged_password;

    // A non-admin isn't allowed to change these fields
    (user_data.user_type.is_none() && user_data.active.is_none() && user_data.flat_id.is_none())
        .err_with(|| {
            "Don't manipulate the user-type, active-Flag or flat-ID"
                .into_redirect_flash(uri!(get_change: id))
        })?;

    user_data
        .name
        .is_empty()
        .not()
        .err_with(|| "Name is empty".into_redirect_flash(uri!(get_change: id)))?;

    // If the password is updated, the two fields must be the same
    (unchanged_password || user_data.pw == user_data.pw_repeat)
        .err_with(|| "Passwords are not the same".into_redirect_flash(uri!(get_change: id)))?;

    if changed_password {
        UserGuard::check_password(&user_data.pw)
            .map_err(|e| e.into_redirect_flash(uri!(get_change: id)))?;
    }

    let entry = user_data.into_inner().into_entry(id);

    match changed_password {
        true => entry.update_unprivileged(&conn),
        false => entry.update_unprivileged_without_password(&conn),
    }
    .and_then(|_| UserSessionEntry::delete_by_user(&conn, user_guard.user.get_id()))
    .map_err(|e| e.into_redirect_flash(uri!(get_change: id)))?;

    Ok(Flash::success(
        Redirect::to(uri!(get_login)),
        "Your account has been successfully updated. Please log in again.".to_string(),
    ))
}
