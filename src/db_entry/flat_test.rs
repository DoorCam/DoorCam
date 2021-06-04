use super::*;

impl Default for FlatEntry<()> {
    fn default() -> Self {
        Self {
            id: (),
            name: "Home".to_string(),
            active: false,
            bell_button_pin: 0,
            local_address: "doorcam.local".to_string(),
            broker_address: "mqtt.local".to_string(),
            broker_port: 1883,
            bell_topic: "/door/bell".to_string(),
            broker_user: "doorbell".to_string(),
            broker_password: "123cdef".to_string(),
            broker_password_iv: "123456789abcdef123456789abcdef".to_string(),
        }
    }
}

impl Default for FlatEntry {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Home".to_string(),
            active: false,
            bell_button_pin: 0,
            local_address: "doorcam.local".to_string(),
            broker_address: "mqtt.local".to_string(),
            broker_port: 1883,
            bell_topic: "/door/bell".to_string(),
            broker_user: "doorbell".to_string(),
            broker_password: "123cdef".to_string(),
            broker_password_iv: "123456789abcdef123456789abcdef".to_string(),
        }
    }
}

#[test]
fn scenario_1_with_all_methods() {
    let sql_scheme = include_str!("../../scheme.sql");
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(sql_scheme).unwrap();

    let mut flat = FlatEntry::default().create(&conn).unwrap();

    let flats = FlatEntry::get_all(&conn).unwrap();
    assert_eq!(flats.len(), 1);
    assert!(flats.contains(&flat));

    flat.broker_password = "deadbeef".to_string();
    flat.update(&conn).unwrap();

    assert_eq!(
        FlatEntry::get_by_id(&conn, flat.get_id()).unwrap().unwrap(),
        flat
    );

    assert!(FlatEntry::get_active(&conn).unwrap().is_empty());

    flat.active = true;
    flat.update_without_password(&conn).unwrap();

    let active_flats = FlatEntry::get_active(&conn).unwrap();
    assert_eq!(active_flats.len(), 1);
    assert!(active_flats.contains(&flat));

    flat.delete(&conn).unwrap();

    assert!(FlatEntry::get_all(&conn).unwrap().is_empty());
}
