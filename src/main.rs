#![warn(clippy::use_self)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;

#[cfg(test)]
#[macro_use]
extern crate matches;

#[macro_use]
extern crate lazy_static;

use rsevents::{AutoResetEvent, State};
use std::sync::{Arc, Mutex};

use rocket_contrib::databases::rusqlite;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod db_entry;
mod requests;
mod template_contexts;
mod utils;

mod iot;

use utils::config::CONFIG;

fn main() {
    #[cfg(not(debug_assertions))]
    log4rs::init_file("logger.yaml", Default::default()).unwrap();

    // IoT event_loop
    let flat_sync_event = Arc::new(AutoResetEvent::new(State::Unset));
    let db = match rusqlite::Connection::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Can't establish db connection: {}", e);
            log::error!("Can't establish db connection: {}", e);
            return;
        }
    };
    iot::event_loop(&flat_sync_event, db);

    // Web
    rocket::ignite()
        .mount(
            "/",
            routes![
                requests::index_view::get_user_index_view,
                requests::index_view::redirect_admin_to_index,
                requests::index_view::get_admin_index_view,
                requests::index_view::get_not_found,
                requests::user_auth::get_login,
                requests::user_auth::post_login_data,
                requests::user_auth::get_logout,
                requests::user::get_users,
                requests::user::get_create,
                requests::user::post_create_data,
                requests::user::get_change,
                requests::user::user_post_change_data,
                requests::user::admin_post_change_data,
                requests::user::delete,
                requests::flat::get_flats,
                requests::flat::get_create,
                requests::flat::post_create_data,
                requests::flat::get_change,
                requests::flat::post_change_data,
                requests::flat::delete,
                requests::door::get_open_door,
            ],
        )
        .register(catchers![requests::index_view::not_found_catcher])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .attach(db_entry::DbConn::fairing())
        .attach(SpaceHelmet::default())
        .manage(Mutex::new(iot::DoorControl::new(
            CONFIG.iot.door_opener_pin,
        )))
        .manage(flat_sync_event)
        .launch();
}
