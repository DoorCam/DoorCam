use super::*;
use rocket::local::Client;
use rocket_contrib::templates::{tera::Context, Template};

#[test]
fn main_view_without_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "main_view",
        &MainViewContext {
            message: None,
            cam_url: String::new(),
            change_user_url: String::new(),
            activate_door_url: String::new(),
            logout_url: String::new(),
        },
    )
    .unwrap();
}

#[test]
fn main_view_with_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "main_view",
        &MainViewContext {
            message: Some(Message::error("21".to_string())),
            cam_url: String::new(),
            change_user_url: String::new(),
            activate_door_url: String::new(),
            logout_url: String::new(),
        },
    )
    .unwrap();
}

#[test]
fn admin_view_without_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "admin_view",
        &AdminViewContext {
            message: None,
            nav: AdminNav::new(),
        },
    )
    .unwrap();
}

#[test]
fn admin_view_with_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "admin_view",
        &AdminViewContext {
            message: Some(Message::error("21".to_string())),
            nav: AdminNav::new(),
        },
    )
    .unwrap();
}

#[test]
fn login_without_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(client.rocket(), "login", &LoginContext { message: None }).unwrap();
}

#[test]
fn login_with_message() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "login",
        &LoginContext {
            message: Some(Message::error("21".to_string())),
        },
    )
    .unwrap();
}

#[test]
fn catcher_of_404() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(client.rocket(), "404", &Context::new()).unwrap();
}
