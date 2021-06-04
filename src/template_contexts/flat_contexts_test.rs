use super::*;
use rocket::local::Client;
use rocket_contrib::templates::Template;

#[test]
fn user_overview_with_empty_view() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_overview",
        &FlatOverviewContext::view(Vec::new(), None),
    )
    .unwrap();
}

#[test]
fn user_overview_with_view() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_overview",
        &FlatOverviewContext::view(vec![FlatEntry::default()], None),
    )
    .unwrap();
}

#[test]
fn user_overview_with_view_and_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_overview",
        &FlatOverviewContext::view(
            vec![FlatEntry::default()],
            Some(Message::error("21".to_string())),
        ),
    )
    .unwrap();
}

#[test]
fn user_overview_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_overview",
        &FlatOverviewContext::error(Message::error("21".to_string())),
    )
    .unwrap();
}

#[test]
fn user_details_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_details",
        &FlatDetailsContext::error(Message::error("21".to_string())),
    )
    .unwrap();
}

#[test]
fn user_create() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_details",
        &FlatDetailsContext::create(None),
    )
    .unwrap();
}

#[test]
fn user_create_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_details",
        &FlatDetailsContext::create(Some(Message::error("21".to_string()))),
    )
    .unwrap();
}

#[test]
fn user_change() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_details",
        &FlatDetailsContext::change(None, FlatEntry::default()),
    )
    .unwrap();
}

#[test]
fn user_change_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "flat_details",
        &FlatDetailsContext::change(Some(Message::error("21".to_string())), FlatEntry::default()),
    )
    .unwrap();
}
