//! All structs and functions used to communicate with the database and represent the data.

pub use rocket_contrib::databases::rusqlite;

#[database("sqlite_db")]
#[derive(Clone)]
pub struct DbConn(rusqlite::Connection);

mod user;
pub use user::{HashEntry, UserEntry};
mod user_type;
pub use user_type::UserType;

mod flat;
pub use flat::FlatEntry;
