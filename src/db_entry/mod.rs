//! All structs and functions used to communicate with the database and represent the data.

pub use rocket_contrib::databases::rusqlite;

#[database("sqlite_db")]
#[derive(Clone)]
pub struct DbConn(rusqlite::Connection);

pub trait Identifier {}

impl Identifier for () {}
impl Identifier for u32 {}

mod user;
pub use user::{HashEntry, UserEntry};
mod user_type;
pub use user_type::UserType;

mod flat;
pub use flat::FlatEntry;
