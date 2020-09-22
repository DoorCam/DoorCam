use crate::guards::OnlyUserGuard;
use rocket::response::Flash;
use rocket::State;

#[get("/api/door/open")]
pub fn get_door_open<'r>(
    _user: OnlyUserGuard,
    door_ctrl: State<'r, std::sync::Mutex<crate::iot::DoorControl>>,
) -> Flash<()> {
    let mut ctrl = match door_ctrl.lock() {
        Ok(ctrl) => ctrl,
        Err(e) => return Flash::error((), e.to_string()),
    };
    if let Err(e) = ctrl.open() {
        return Flash::error((), e.to_string());
    }
    return Flash::success((), "Door opened");
}
