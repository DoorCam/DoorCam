use super::*;

#[test]
fn start_closed() {
    let mut ctrl = DoorControl::new(0);
    assert_matches!(ctrl.is_opener_active(), Ok(false));
}

#[test]
fn open() {
    let mut ctrl = DoorControl::new(0);
    assert_matches!(ctrl.activate_opener(), Ok(()));
    assert_matches!(ctrl.is_opener_active(), Ok(true));
}

#[test]
fn auto_stop_opening() {
    let mut ctrl = DoorControl::new(0);
    assert_matches!(ctrl.activate_opener(), Ok(()));
    std::thread::sleep(std::time::Duration::from_secs(
        CONFIG.iot.door_opening_time.as_secs() + 1,
    ));
    assert_matches!(ctrl.is_opener_active(), Ok(false));
}
