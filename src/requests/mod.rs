pub mod door;
pub mod flat;
pub mod index_view;
pub mod user;
pub mod user_auth;

trait FormToEntry<T> {
    fn to_entry(self, id: u32) -> T;
}
