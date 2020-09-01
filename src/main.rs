#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct CamContext {
    cam_url: String
}

#[get("/")]
fn index() -> Template {
    let context = CamContext{cam_url: "http://doorcam.fritz.box:8081/".to_string()};
    Template::render("cam", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}