use super::{ErrorIntoFlash, FormIntoEntry, ResultFlash};
use crate::db_entry::{DbConn, Entry, FlatEntry};
use crate::template_contexts::{FlatDetailsContext, FlatOverviewContext, Message};
use crate::utils::crypto;
use crate::utils::guards::AdminGuard;
use bool_ext::BoolExt;
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use rsevents::AutoResetEvent;
use std::ops::Not;
use std::sync::Arc;

/// Struct which retrieves all form data from the flat details.
#[derive(FromForm)]
pub struct FlatForm {
    name: String,
    active: bool,
    bell_button_pin: u8,
    local_address: String,
    broker_address: String,
    broker_port: u16,
    bell_topic: String,
    tamper_alarm_topic: String,
    broker_user: String,
    broker_password: String,
}

impl FlatForm {
    fn encrypt(&self) -> (String, String) {
        let mut broker_pw_iv = [0; 16];
        crypto::fill_rand_array(&mut broker_pw_iv);
        let encrypted_broker_password = base64::encode(crypto::symetric_encrypt(
            &crate::CONFIG.security.encryption_key,
            &broker_pw_iv,
            self.broker_password.as_bytes(),
        ));
        let broker_pw_iv = base64::encode(broker_pw_iv);
        (broker_pw_iv, encrypted_broker_password)
    }
}

impl FormIntoEntry<FlatEntry<()>, FlatEntry> for FlatForm {
    fn into_insertable(self) -> FlatEntry<()> {
        let (broker_pw_iv, encrypted_broker_password) = self.encrypt();

        FlatEntry {
            id: (),
            name: self.name,
            active: self.active,
            bell_button_pin: self.bell_button_pin,
            local_address: self.local_address,
            broker_address: self.broker_address,
            broker_port: self.broker_port,
            bell_topic: self.bell_topic,
            tamper_alarm_topic: self
                .tamper_alarm_topic
                .is_empty()
                .not()
                .some(self.tamper_alarm_topic),
            broker_user: self.broker_user,
            broker_password: encrypted_broker_password,
            broker_password_iv: broker_pw_iv,
        }
    }

    fn into_entry(self, id: u32) -> FlatEntry {
        let (broker_pw_iv, encrypted_broker_password) = self.encrypt();

        FlatEntry {
            id,
            name: self.name,
            active: self.active,
            bell_button_pin: self.bell_button_pin,
            local_address: self.local_address,
            broker_address: self.broker_address,
            broker_port: self.broker_port,
            bell_topic: self.bell_topic,
            tamper_alarm_topic: self
                .tamper_alarm_topic
                .is_empty()
                .not()
                .some(self.tamper_alarm_topic),
            broker_user: self.broker_user,
            broker_password: encrypted_broker_password,
            broker_password_iv: broker_pw_iv,
        }
    }
}

/// get form to create a flat
#[get("/admin/flat/create")]
pub fn get_create(_admin: AdminGuard, flash: Option<FlashMessage>) -> Template {
    let context = FlatDetailsContext::create(flash.map(Message::from));
    Template::render("flat_details", &context)
}

/// post flat-data to create a flat
#[post("/admin/flat/create", data = "<flat_data>")]
pub fn post_create_data(
    flat_data: Form<FlatForm>,
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
) -> Result<Redirect, Flash<Redirect>> {
    (flat_data.name.is_empty()
        || flat_data.local_address.is_empty()
        || flat_data.broker_address.is_empty()
        || flat_data.bell_topic.is_empty()
        || flat_data.broker_user.is_empty()
        || flat_data.broker_password.is_empty())
    .not()
    .err_with(|| "Mandatory field is empty".into_redirect_flash(uri!(get_create)))?;

    flat_data
        .into_inner()
        .into_insertable()
        .create(&conn)
        .map_err(|e| e.into_redirect_flash(uri!(get_create)))?;

    // sync iot::EventHandler
    flat_sync_event.set();

    return Ok(Redirect::to(uri!(get_flats)));
}

/// get all flats
#[get("/admin/flat")]
pub fn get_flats(_admin: AdminGuard, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let context = match FlatEntry::get_all(&conn) {
        Ok(flats) => FlatOverviewContext::view(flats, flash.map(Message::from)),
        Err(e) => FlatOverviewContext::error(Message::error(format!("DB Error: {}", e))),
    };
    Template::render("flat_overview", &context)
}

/// delete a flat by id
#[delete("/admin/flat/delete/<id>")]
pub fn delete(
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
    id: u32,
) -> ResultFlash<()> {
    FlatEntry::delete_entry(&conn, id).map_err(|e| e.into_flash())?;

    // sync iot::EventHandler
    flat_sync_event.set();

    Ok(Flash::success((), "Flat deleted"))
}

/// get the form for modifying a flat
#[get("/admin/flat/change/<id>")]
pub fn get_change(
    _admin: AdminGuard,
    conn: DbConn,
    flash: Option<FlashMessage>,
    id: u32,
) -> Result<Template, Status> {
    // get the FlatEntry to show its values
    let context = match FlatEntry::get_by_id(&conn, id) {
        Ok(Some(flat)) => FlatDetailsContext::change(flash.map(Message::from), flat),
        Ok(None) => FlatDetailsContext::error(Message::error("No flat found".to_string())),
        Err(e) => FlatDetailsContext::error(Message::error(e.to_string())),
    };
    Ok(Template::render("flat_details", &context))
}

/// post the form-data to modify the flat
#[post("/admin/flat/change/<id>", data = "<flat_data>")]
pub fn post_change_data(
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
    id: u32,
    flat_data: Form<FlatForm>,
) -> Result<Redirect, Flash<Redirect>> {
    (flat_data.name.is_empty()
        || flat_data.local_address.is_empty()
        || flat_data.broker_address.is_empty()
        || flat_data.bell_topic.is_empty()
        || flat_data.broker_user.is_empty())
    .not()
    .err_with(|| "Mandatory field is empty".into_redirect_flash(uri!(get_create)))?;

    let update_password = !flat_data.broker_password.is_empty();
    let flat = flat_data.into_inner().into_entry(id);

    let update_result = match update_password {
        true => flat.update(&conn),
        false => flat.update_without_password(&conn),
    };

    update_result.map_err(|e| e.into_redirect_flash(uri!(get_change: id)))?;

    // sync iot::EventHandler
    flat_sync_event.set();

    return Ok(Redirect::to(uri!(get_flats)));
}
