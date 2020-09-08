#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod crypto;

mod db_conn;

mod guards;

mod index_view;

mod error_request;

mod user_entry;

mod user_auth;
mod user_requests;

mod template_contexts;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index_view::index,
                error_request::error,
                user_auth::get_login,
                user_auth::post_login_data,
                user_requests::get_users,
                user_requests::get_create,
                user_requests::post_create_data
            ],
        )
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .attach(db_conn::DbConn::fairing())
        .launch();
}
