use super::{index_view::*, ErrorIntoFlash, ResultFlash};
use crate::utils::guards::OnlyUserGuard;
use rocket::response::{Flash, Redirect};
use rocket::State;

/// Get to activate the door opener
#[get("/api/door/activate")]
pub fn get_open_door(
    _user: OnlyUserGuard,
    door_ctrl: State<crate::iot::DoorControl>,
) -> ResultFlash<Redirect> {
    door_ctrl
        .activate_opener()
        .map_err(|e| e.into_redirect_flash(uri!(get_user_index_view)))?;

    Ok(Flash::success(
        Redirect::to(uri!(get_user_index_view)),
        "Door opened",
    ))
}
