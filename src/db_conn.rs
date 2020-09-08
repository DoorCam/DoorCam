pub use rocket_contrib::databases::rusqlite;

#[database("sqlite_db")]
pub struct DbConn(rusqlite::Connection);
