#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod crypto;
mod db_conn;
mod guards;
mod index_view;
mod template_contexts;
mod user_auth;
mod user_entry;
mod user_requests;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index_view::index,
                index_view::get_not_found,
                user_auth::get_login,
                user_auth::post_login_data,
                user_auth::get_logout,
                user_requests::get_users,
                user_requests::get_create,
                user_requests::post_create_data,
                user_requests::get_change_user,
                user_requests::delete_user,
            ],
        )
        .register(catchers![index_view::not_found_catcher])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .attach(db_conn::DbConn::fairing())
        .attach(SpaceHelmet::default())
        .launch();
}
