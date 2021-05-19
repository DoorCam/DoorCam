use super::*;
use crate::db_entry::{HashEntry, UserEntry, UserType};

#[test]
fn scenario_1_with_all_methods() {
    let sql_scheme = include_str!("../../scheme.sql");
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(sql_scheme).unwrap();

    let mut session = UserSessionEntry {
        id: (),
        login_datetime: Utc::now(),
        user: 0,
    }
    .create(&conn)
    .unwrap();

    let sessions = UserSessionEntry::get_all(&conn).unwrap();
    assert_eq!(sessions.len(), 1);
    assert!(sessions.contains(&session));

    session.login_datetime = Utc::now();
    session.update(&conn).unwrap();

    let other_user = UserEntry::<(), u32> {
        id: (),
        name: String::new(),
        pw_hash: HashEntry {
            hash: String::new(),
            salt: String::new(),
            config: String::new(),
        },
        user_type: UserType::User,
        active: true,
        flat: None,
    }
    .create(&conn)
    .unwrap();

    UserSessionEntry {
        id: (),
        login_datetime: Utc::now(),
        user: other_user.get_id(),
    }
    .create(&conn)
    .unwrap();
    UserSessionEntry {
        id: (),
        login_datetime: Utc::now(),
        user: other_user.get_id(),
    }
    .create(&conn)
    .unwrap();

    assert_eq!(UserSessionEntry::get_all(&conn).unwrap().len(), 3);

    UserSessionEntry::delete_by_user(&conn, other_user.get_id()).unwrap();

    assert_eq!(UserSessionEntry::get_all(&conn).unwrap().len(), 1);

    assert_eq!(
        UserSessionEntry::get_by_id(&conn, session.get_id())
            .unwrap()
            .unwrap(),
        session
    );

    session.delete(&conn).unwrap();

    assert!(UserSessionEntry::get_all(&conn).unwrap().is_empty());
}
