use super::*;
use crate::db_entry::UserEntry;

impl Default for UserSessionEntry<(), u32> {
    fn default() -> Self {
        Self {
            id: (),
            login_datetime: Utc::now(),
            user: 0,
        }
    }
}

impl Default for UserSessionEntry {
    fn default() -> Self {
        Self {
            id: 1,
            login_datetime: Utc::now(),
            user: 0,
        }
    }
}

#[test]
fn scenario_1_with_all_methods() {
    let sql_scheme = include_str!("../../scheme.sql");
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(sql_scheme).unwrap();

    let mut session = UserSessionEntry::default().create(&conn).unwrap();

    let sessions = UserSessionEntry::get_all(&conn).unwrap();
    assert_eq!(sessions.len(), 1);
    assert!(sessions.contains(&session));

    session.login_datetime = Utc::now();
    session.update(&conn).unwrap();

    let other_user = UserEntry::default().create(&conn).unwrap();

    UserSessionEntry {
        user: other_user.get_id(),
        ..Default::default()
    }
    .create(&conn)
    .unwrap();
    UserSessionEntry {
        user: other_user.get_id(),
        ..Default::default()
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
