use crate::guards::OnlyUserGuard;
use rocket::response::Flash;
use rocket::State;

#[cfg(feature = "only_web")]
#[get("/api/door/open")]
pub fn get_door_open(_user: OnlyUserGuard) -> Flash<()> {
    Flash::success((), "Door opened");
}

#[cfg(feature = "iot")]
#[get("/api/door/open")]
pub fn get_door_open<'r>(
    _user: OnlyUserGuard,
    door_ctrl: State<'r, std::sync::Mutex<crate::iot::DoorControl>>,
) -> Flash<()> {
    match door_ctrl.lock() {
        Ok(mut ctrl) => ctrl.open(),
        Err(e) => return Flash::error((), e.to_string()),
    }
    return Flash::success((), "Door opened");
}
