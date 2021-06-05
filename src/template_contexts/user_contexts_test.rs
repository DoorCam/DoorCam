use super::*;
use rocket::local::Client;
use rocket_contrib::templates::Template;

#[test]
fn user_overview_with_empty_view() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_overview",
        &UserOverviewContext::view(Vec::new(), None),
    )
    .unwrap();
}

#[test]
fn user_overview_with_view() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_overview",
        &UserOverviewContext::view(vec![UserEntry::default()], None),
    )
    .unwrap();
}

#[test]
fn user_overview_with_view_and_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_overview",
        &UserOverviewContext::view(
            vec![UserEntry::default()],
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
        "user_overview",
        &UserOverviewContext::error(Message::error("21".to_string())),
    )
    .unwrap();
}

#[test]
fn user_details_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::error(Message::error("21".to_string())),
    )
    .unwrap();
}

#[test]
fn user_create() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::create(None, vec![FlatEntry::default()]),
    )
    .unwrap();
}

#[test]
fn user_create_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::create(Some(Message::error("21".to_string())), Vec::new()),
    )
    .unwrap();
}

#[test]
fn user_change_by_user() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::change(
            None,
            false,
            UserEntry::default(),
            vec![FlatEntry::default()],
        ),
    )
    .unwrap();
}

#[test]
fn user_change_by_user_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::change(
            Some(Message::error("21".to_string())),
            false,
            UserEntry::default(),
            Vec::new(),
        ),
    )
    .unwrap();
}

#[test]
fn user_change_by_admin() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::change(None, true, UserEntry::default(), vec![FlatEntry::default()]),
    )
    .unwrap();
}

#[test]
fn user_change_by_admin_with_error() {
    let rocket = rocket::ignite().attach(Template::fairing());
    let client = Client::new(rocket).expect("valid rocket");

    Template::show(
        client.rocket(),
        "user_details",
        &UserDetailsContext::change(
            Some(Message::error("21".to_string())),
            true,
            UserEntry::default(),
            Vec::new(),
        ),
    )
    .unwrap();
}
