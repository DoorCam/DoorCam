use super::FormToEntry;
use crate::db_entry::{DbConn, FlatEntry};
use crate::guards::AdminGuard;
use crate::template_contexts::{FlatDetailsContext, FlatOverviewContext, Message};
use rocket::http::Status;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use rsevents::AutoResetEvent;
use std::sync::Arc;

#[derive(FromForm)]
pub struct FlatForm {
    name: String,
    active: bool,
    bell_button_pin: u8,
    local_address: String,
    broker_address: String,
    broker_port: u16,
    bell_topic: String,
}

impl FormToEntry<FlatEntry> for FlatForm {
    fn to_entry(self, id: u32) -> FlatEntry {
        FlatEntry {
            id,
            name: self.name,
            active: self.active,
            bell_button_pin: self.bell_button_pin,
            local_address: self.local_address,
            broker_address: self.broker_address,
            broker_port: self.broker_port,
            bell_topic: self.bell_topic,
        }
    }
}

#[get("/admin/flat/create")]
pub fn get_create(_admin: AdminGuard, flash: Option<FlashMessage>) -> Template {
    let context = FlatDetailsContext::create(flash.map(Message::from));
    Template::render("flat_details", &context)
}

#[post("/admin/flat/create", data = "<flat_data>")]
pub fn post_create_data(
    flat_data: Form<FlatForm>,
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
) -> Result<Redirect, Flash<Redirect>> {
    if flat_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            "Name is empty",
        ));
    }
    if let Err(e) = FlatEntry::create(
        &conn,
        &flat_data.name,
        flat_data.active,
        flat_data.bell_button_pin,
        &flat_data.local_address,
        &flat_data.broker_address,
        flat_data.broker_port,
        &flat_data.bell_topic,
    ) {
        return Err(Flash::error(
            Redirect::to(uri!(get_create)),
            format!("DB Error: {}", e),
        ));
    }

    // sync iot::EventHandler
    flat_sync_event.set();

    return Ok(Redirect::to(uri!(get_flats)));
}

#[get("/admin/flat")]
pub fn get_flats(_admin: AdminGuard, conn: DbConn) -> Template {
    let context = match FlatEntry::get_all(&conn) {
        Ok(flats) => FlatOverviewContext::view(flats),
        Err(e) => FlatOverviewContext::error(Message::error(format!("DB Error: {}", e))),
    };
    Template::render("flat_overview", &context)
}

#[delete("/admin/flat/delete/<id>")]
pub fn delete(
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
    id: u32,
) -> Flash<()> {
    if let Err(e) = FlatEntry::delete(&conn, id) {
        return Flash::error((), e.to_string());
    };

    // sync iot::EventHandler
    flat_sync_event.set();

    Flash::success((), "Flat deleted")
}

#[get("/admin/flat/change/<id>")]
pub fn get_change(
    _admin: AdminGuard,
    conn: DbConn,
    flash: Option<FlashMessage>,
    id: u32,
) -> Result<Template, Status> {
    let context = match FlatEntry::get_by_id(&conn, id).as_mut() {
        Ok(flats) => match flats.pop() {
            Some(flat) => FlatDetailsContext::change(flash.map(Message::from), flat),
            None => FlatDetailsContext::error(Message::error("No flat found".to_string())),
        },
        Err(e) => FlatDetailsContext::error(Message::error(e.to_string())),
    };
    Ok(Template::render("flat_details", &context))
}

#[post("/admin/flat/change/<id>", data = "<flat_data>")]
pub fn post_change_data(
    _admin: AdminGuard,
    conn: DbConn,
    flat_sync_event: State<Arc<AutoResetEvent>>,
    id: u32,
    flat_data: Form<FlatForm>,
) -> Result<Redirect, Flash<Redirect>> {
    if flat_data.name.is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            "Name is empty",
        ));
    }
    if let Err(e) = flat_data.into_inner().to_entry(id).change(&conn) {
        return Err(Flash::error(
            Redirect::to(uri!(get_change: id)),
            format!("DB Error: {}", e),
        ));
    }

    // sync iot::EventHandler
    flat_sync_event.set();

    return Ok(Redirect::to(uri!(get_flats)));
}
