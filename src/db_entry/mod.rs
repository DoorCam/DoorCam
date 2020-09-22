pub use rocket_contrib::databases::rusqlite;

#[database("sqlite_db")]
pub struct DbConn(rusqlite::Connection);

mod user;
pub use user::{UserEntry, HashEntry};
