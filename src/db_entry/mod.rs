//! All structs and functions used to communicate with the database and represent the data.

pub use rocket_contrib::databases::rusqlite;
pub use rusqlite::Connection;

#[cfg(not(test))]
#[database("sqlite_db")]
#[derive(Clone)]
pub struct DbConn(Connection);

#[cfg(test)]
mod connection_test;
#[cfg(test)]
pub use connection_test::DbConn;

/// Used for the `ID` field in `Entries` to statically differentiate between insertable and known
/// `Entries`.
/// ```
/// struct TableEntry<ID: Identifier = u32> {
///     id: ID,
///     field: u32,
/// }
///
/// impl TableEntry<()> {
///     pub fn insert(&self) -> TableEntry { todo!() }
/// }
///
/// impl TableEntry<u32> {
///     pub fn get() -> Vec<Self> { todo!() }
/// }
/// ```
pub trait Identifier {}

/// Used for the `ID` fields of insertable `Entries`.
impl Identifier for () {}
/// Used for the `ID` fields of known `Entries`.
impl Identifier for u32 {}

/// A generic trait for `DB` `Entries` to abstract over database concepts like foreign key.
/// ```
/// struct TableEntry<FKey: Entry = ForeignEntry> {
///     field: FKey,
/// }
///
/// impl<FKey: Entry> TableEntry<FKey> {
///     pub fn insert(&self) -> TableEntry { todo!() }
/// }
///
/// impl TableEntry<ForeignEntry> {
///     pub fn get() -> Vec<Self> { todo!() }
/// }
/// ```
pub trait Entry: Clone {
    fn get_id(&self) -> u32;
    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error>;
    fn delete_entry(conn: &Connection, id: u32) -> Result<(), rusqlite::Error>;
    fn delete(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        Self::delete_entry(conn, self.get_id())
    }
}

/// Generic implementation for `u32` as a foreign key ID.
impl Entry for u32 {
    fn get_id(&self) -> u32 {
        *self
    }
    fn update(&self, _conn: &Connection) -> Result<(), rusqlite::Error> {
        unreachable!();
    }
    fn delete_entry(_conn: &Connection, _id: u32) -> Result<(), rusqlite::Error> {
        unreachable!();
    }
}

mod user;
pub use user::{HashEntry, UserEntry};
mod user_type;
pub use user_type::UserType;

mod flat;
pub use flat::FlatEntry;

mod user_session;
pub use user_session::UserSessionEntry;
