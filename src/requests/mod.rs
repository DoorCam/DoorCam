//! All user-facing logic.

use crate::db_entry::Entry;
use rocket::http::uri::Origin;
use rocket::response::{Flash, Redirect};

pub mod door;
pub mod flat;
pub mod index_view;
pub mod user;
pub mod user_auth;

type ResultFlash<T> = Result<Flash<T>, Flash<T>>;

trait ErrorIntoFlash {
    fn into_redirect_flash(self, uri: Origin<'static>) -> Flash<Redirect>;
    fn into_flash(self) -> Flash<()>;
}

impl<T: ToString> ErrorIntoFlash for T {
    fn into_redirect_flash(self, uri: Origin<'static>) -> Flash<Redirect> {
        Flash::error(Redirect::to(uri), self.to_string())
    }
    fn into_flash(self) -> Flash<()> {
        Flash::error((), self.to_string())
    }
}

/// Trait which transforms form-data to entry-data using an additional id-field
trait FormIntoEntry<I, E: Entry> {
    fn into_insertable(self) -> I;
    fn into_entry(self, id: u32) -> E;
}
