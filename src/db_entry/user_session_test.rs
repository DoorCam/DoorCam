use super::*;

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

    assert_eq!(
        UserSessionEntry::get_by_id(&conn, session.get_id())
            .unwrap()
            .unwrap(),
        session
    );

    session.delete(&conn).unwrap();

    assert!(UserSessionEntry::get_all(&conn).unwrap().is_empty());
}
