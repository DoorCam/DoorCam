//! All user-facing logic.

use crate::db_entry::Entry;
use easy_ext::ext;
use rocket::http::uri::Origin;
use rocket::response::{Flash, Redirect};

pub mod door;
pub mod flat;
pub mod index_view;
pub mod user;
pub mod user_auth;

type ResultFlash<T> = Result<Flash<T>, Flash<T>>;

#[ext(ErrorIntoFlash)]
impl<T: ToString> T {
    fn into_redirect_flash(self, uri: Origin<'static>) -> Flash<Redirect> {
        Flash::error(Redirect::to(uri), self.to_string())
    }
    fn into_flash(self) -> Flash<()> {
        Flash::error((), self.to_string())
    }
}

/// Trait which transforms form-data to entry-data using an additional id-field
trait FormIntoEntry<I, E: Entry> {
    type Error;
    fn into_insertable(self) -> Result<I, Self::Error>;
    fn into_entry(self, id: u32) -> Result<E, Self::Error>;
}
