use super::*;
use crate::db_entry::{HashEntry, UserType};
use rocket::http::Cookie;
use rocket::local::Client;

fn get_session() -> UserSessionEntry {
    UserSessionEntry {
        id: 0,
        login_datetime: Utc::now(),
        user: 0,
    }
}

fn get_user() -> UserEntry {
    UserEntry {
        user_type: UserType::User,
        id: 0,
        name: String::new(),
        active: true,
        flat: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    }
}

fn get_user_guard() -> UserGuard {
    UserGuard {
        user: get_user(),
        session: get_session(),
    }
}

fn get_admin() -> UserEntry {
    UserEntry {
        user_type: UserType::Admin,
        id: 0,
        name: String::new(),
        active: true,
        flat: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    }
}

fn get_admin_guard() -> UserGuard {
    UserGuard {
        user: get_admin(),
        session: get_session(),
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
    let user = serde_json::to_string(&get_user_guard()).expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Success(_)
    );
}

#[test]
fn admin_on_only_user_guard() {
    let user = serde_json::to_string(&get_admin_guard()).expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Forward(_)
    );
}

#[test]
fn user_on_admin_guard() {
    let user = serde_json::to_string(&get_user_guard()).expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Forward(_));
}

#[test]
fn admin_on_admin_guard() {
    let user = serde_json::to_string(&get_admin_guard()).expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client
        .get("/")
        .private_cookie(Cookie::new("user_session_guard", user));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Success(_));
}
