///All user-facing logic.
pub mod door;
pub mod flat;
pub mod index_view;
pub mod user;
pub mod user_auth;

/// Trait which transforms form-data to entry-data using an additional id-field
trait FormIntoEntry<T> {
    fn into_entry(self, id: u32) -> T;
}
