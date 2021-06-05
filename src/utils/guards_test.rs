use super::*;
use crate::db_entry::UserType;
use rocket::http::Cookie;
use rocket::local::Client;

fn get_session() -> UserSessionEntry {
    UserSessionEntry {
        id: 0,
        user: 0,
        ..Default::default()
    }
}

fn create_session(conn: &DbConn) -> UserSessionEntry {
    UserSessionEntry {
        user: 0,
        ..Default::default()
    }
    .create(&conn)
    .unwrap()
}

fn get_user() -> UserEntry {
    UserEntry {
        id: 0,
        user_type: UserType::User,
        ..Default::default()
    }
}

fn get_user_guard() -> UserGuard {
    UserGuard {
        user: get_user(),
        session: get_session(),
    }
}

fn create_user_guard(conn: &DbConn) -> UserGuard {
    UserGuard {
        user: get_user(),
        session: create_session(conn),
    }
}

fn get_admin() -> UserEntry {
    UserEntry {
        id: 0,
        user_type: UserType::Admin,
        ..Default::default()
    }
}

fn get_admin_guard() -> UserGuard {
    UserGuard {
        user: get_admin(),
        session: get_session(),
    }
}

fn create_admin_guard(conn: &DbConn) -> UserGuard {
    UserGuard {
        user: get_admin(),
        session: create_session(conn),
    }
}

#[test]
fn user_is_user() {
    assert!(get_user_guard().is_user());
}

#[test]
fn user_is_not_admin() {
    assert!(!get_user_guard().is_admin());
}

#[test]
fn admin_is_not_user() {
    assert!(!get_admin_guard().is_user());
}

#[test]
fn admin_is_admin() {
    assert!(get_admin_guard().is_admin());
}

#[test]
fn user_on_only_user_guard() {
    let server = rocket::ignite().attach(DbConn::fairing());
    let conn = DbConn::get_one(&server).unwrap();

    let user_session =
        serde_json::to_string(&create_user_guard(&conn)).expect("serialization error");

    let client = Client::new(server).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user_session));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Success(_)
    );
}

#[test]
fn admin_on_only_user_guard() {
    let server = rocket::ignite().attach(DbConn::fairing());
    let conn = DbConn::get_one(&server).unwrap();

    let user_session =
        serde_json::to_string(&create_admin_guard(&conn)).expect("serialization error");

    let client = Client::new(server).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user_session));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Forward(_)
    );
}

#[test]
fn user_on_admin_guard() {
    let server = rocket::ignite().attach(DbConn::fairing());
    let conn = DbConn::get_one(&server).unwrap();

    let user_session =
        serde_json::to_string(&create_user_guard(&conn)).expect("serialization error");

    let client = Client::new(server).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user_session));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Forward(_));
}

#[test]
fn admin_on_admin_guard() {
    let server = rocket::ignite().attach(DbConn::fairing());
    let conn = DbConn::get_one(&server).unwrap();

    let user_session =
        serde_json::to_string(&create_admin_guard(&conn)).expect("serialization error");

    let client = Client::new(server).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user_session));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Success(_));
}

#[test]
fn unknown_session() {
    let server = rocket::ignite().attach(DbConn::fairing());

    let user_session = serde_json::to_string(&get_user_guard()).expect("serialization error");

    let client = Client::new(server).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user_session));

    assert_matches!(UserGuard::from_request(&req.inner()), Outcome::Forward(_));
}
