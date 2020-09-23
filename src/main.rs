#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;

#[cfg(test)]
#[macro_use]
extern crate matches;

use std::sync::{Arc, Mutex};

use rocket_contrib::databases::rusqlite;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod crypto;
mod db_entry;
mod guards;
mod requests;
mod template_contexts;

mod iot;

fn main() {
    log4rs::init_file("logger.yaml", Default::default()).unwrap();
    let sync_flag = Arc::new(Mutex::new(false));
    let sf = Arc::clone(&sync_flag);
    let db = match rusqlite::Connection::open("db.sqlite") {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Can't establish db connection: {}", e);
            return;
        }
    };
    std::thread::spawn(move || {
        let mut iot_events = iot::EventHandler::new(sf, db);
        iot_events.event_loop();
    });
    rocket::ignite()
        .mount(
            "/",
            routes![
                requests::index_view::index,
                requests::index_view::get_not_found,
                requests::user_auth::get_login,
                requests::user_auth::post_login_data,
                requests::user_auth::get_logout,
                requests::user::get_users,
                requests::user::get_create,
                requests::user::post_create_data,
                requests::user::get_change,
                requests::user::post_change_data,
                requests::user::delete,
                requests::door::get_open_door,
            ],
        )
        .register(catchers![requests::index_view::not_found_catcher])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .attach(db_entry::DbConn::fairing())
        .attach(SpaceHelmet::default())
        .manage(Mutex::new(iot::DoorControl::new(1)))
        .manage(sync_flag)
        .launch();
}
