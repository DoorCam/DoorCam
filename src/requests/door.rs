use super::index_view::*;
use crate::guards::OnlyUserGuard;
use rocket::response::{Flash, Redirect};
use rocket::State;

#[get("/api/door/activate")]
pub fn get_open_door<'r>(
    _user: OnlyUserGuard,
    door_ctrl: State<'r, std::sync::Mutex<crate::iot::DoorControl>>,
) -> Flash<Redirect> {
    let mut ctrl = match door_ctrl.lock() {
        Ok(ctrl) => ctrl,
        Err(e) => return Flash::error(Redirect::to(uri!(index)), e.to_string()),
    };
    if let Err(e) = ctrl.activate_opener() {
        return Flash::error(Redirect::to(uri!(index)), e.to_string());
    }
    return Flash::success(Redirect::to(uri!(index)), "Door opened");
}
