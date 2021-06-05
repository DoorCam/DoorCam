use super::*;

impl Default for UserEntry<(), u32> {
    fn default() -> Self {
        Self {
            id: (),
            name: "Alice".to_string(),
            pw_hash: HashEntry {
                hash: "unsecure".to_string(),
                salt: "salt".to_string(),
                config: "plain".to_string(),
            },
            user_type: UserType::User,
            active: true,
            flat: None,
        }
    }
}

impl Default for UserEntry {
    fn default() -> Self {
        Self {
            id: 42,
            name: "Alice".to_string(),
            pw_hash: HashEntry {
                hash: "unsecure".to_string(),
                salt: "salt".to_string(),
                config: "plain".to_string(),
            },
            user_type: UserType::User,
            active: true,
            flat: None,
        }
    }
}

#[test]
fn scenario_1_with_all_methods() {
    let sql_scheme = include_str!("../../scheme.sql");
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(sql_scheme).unwrap();

    let mut user = UserEntry::default().create(&conn).unwrap();

    let users = UserEntry::get_all(&conn).unwrap();
    assert_eq!(users.len(), 2);
    assert!(users.contains(&user));

    user.pw_hash.hash = "superSecure42".to_string();
    user.update(&conn).unwrap();

    assert_eq!(
        UserEntry::get_by_id(&conn, user.get_id()).unwrap().unwrap(),
        user
    );

    user.user_type = UserType::Admin;
    user.update_without_password(&conn).unwrap();

    assert_eq!(
        UserEntry::get_active_by_name(&conn, &user.name)
            .unwrap()
            .unwrap(),
        user
    );

    user.pw_hash.salt = "random".to_string();
    user.update_unprivileged(&conn).unwrap();

    assert_eq!(
        UserEntry::get_by_id(&conn, user.get_id()).unwrap().unwrap(),
        user
    );

    user.name = "Bob".to_string();
    user.update_unprivileged_without_password(&conn).unwrap();

    assert_eq!(
        UserEntry::get_by_id(&conn, user.get_id()).unwrap().unwrap(),
        user
    );
    user.delete(&conn).unwrap();

    assert_eq!(UserEntry::get_all(&conn).unwrap().len(), 1);
}
