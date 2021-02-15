use super::*;
use crate::db_entry::{HashEntry, UserType};
use rocket::http::Cookie;
use rocket::local::Client;

#[test]
fn user_is_user() {
    let guard = UserGuard {
        user: UserEntry {
            user_type: UserType::User,
            id: 0,
            name: String::new(),
            active: true,
            flat_id: None,
            flat_name: None,
            pw_hash: HashEntry {
                hash: String::new(),
                salt: String::new(),
                config: String::new(),
            },
        },
    };
    assert_eq!(guard.is_user(), true);
}

#[test]
fn user_is_not_admin() {
    let guard = UserGuard {
        user: UserEntry {
            user_type: UserType::User,
            id: 0,
            name: String::new(),
            active: true,
            flat_id: None,
            flat_name: None,
            pw_hash: HashEntry {
                hash: String::new(),
                salt: String::new(),
                config: String::new(),
            },
        },
    };
    assert_eq!(guard.is_admin(), false);
}

#[test]
fn admin_is_not_user() {
    let guard = UserGuard {
        user: UserEntry {
            user_type: UserType::Admin,
            id: 0,
            name: String::new(),
            active: true,
            flat_id: None,
            flat_name: None,
            pw_hash: HashEntry {
                hash: String::new(),
                salt: String::new(),
                config: String::new(),
            },
        },
    };
    assert_eq!(guard.is_user(), false);
}

#[test]
fn admin_is_admin() {
    let guard = UserGuard {
        user: UserEntry {
            user_type: UserType::Admin,
            id: 0,
            name: String::new(),
            active: true,
            flat_id: None,
            flat_name: None,
            pw_hash: HashEntry {
                hash: String::new(),
                salt: String::new(),
                config: String::new(),
            },
        },
    };
    assert_eq!(guard.is_admin(), true);
}

#[test]
fn user_on_only_user_guard() {
    let user = serde_json::to_string(&UserEntry {
        user_type: UserType::User,
        id: 0,
        name: String::new(),
        active: true,
        flat_id: None,
        flat_name: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    })
    .expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client.get("/").private_cookie(Cookie::new("user", user));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Success(_)
    );
}

#[test]
fn admin_on_only_user_guard() {
    let user = serde_json::to_string(&UserEntry {
        user_type: UserType::Admin,
        id: 0,
        name: String::new(),
        active: true,
        flat_id: None,
        flat_name: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    })
    .expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client.get("/").private_cookie(Cookie::new("user", user));

    assert_matches!(
        OnlyUserGuard::from_request(&req.inner()),
        Outcome::Forward(_)
    );
}

#[test]
fn user_on_admin_guard() {
    let user = serde_json::to_string(&UserEntry {
        user_type: UserType::User,
        id: 0,
        name: String::new(),
        active: true,
        flat_id: None,
        flat_name: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    })
    .expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client.get("/").private_cookie(Cookie::new("user", user));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Forward(_));
}

#[test]
fn admin_on_admin_guard() {
    let user = serde_json::to_string(&UserEntry {
        user_type: UserType::Admin,
        id: 0,
        name: String::new(),
        active: true,
        flat_id: None,
        flat_name: None,
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
    })
    .expect("serialization error");

    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client.get("/").private_cookie(Cookie::new("user", user));

    assert_matches!(AdminGuard::from_request(&req.inner()), Outcome::Success(_));
}
