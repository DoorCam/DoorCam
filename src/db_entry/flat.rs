use super::{rusqlite, DbConn};
use serde::{Deserialize, Serialize};

/// Flat entry of the corresponding "flat" table.
#[derive(Debug, Serialize, Deserialize)]
pub struct FlatEntry {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub bell_button_pin: u8,
    pub local_address: String,
    pub broker_address: String,
    pub broker_port: u16,
    pub bell_topic: String,
    pub broker_user: String,
    pub broker_password: String,
}

impl FlatEntry {
    #[allow(clippy::too_many_arguments, clippy::ptr_arg)]
    pub fn create(
        conn: &DbConn,
        name: &String,
        active: bool,
        bell_button_pin: u8,
        local_address: &String,
        broker_address: &String,
        broker_port: u16,
        bell_topic: &String,
        broker_user: &String,
        broker_password: &String,
    ) -> Result<Self, rusqlite::Error> {
        conn.execute(
            "INSERT INTO flat (name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[name, &active, &bell_button_pin, local_address, broker_address, &broker_port, bell_topic, broker_user, broker_password])?;
        Ok(Self {
            id: (conn.last_insert_rowid() as u32),
            name: name.clone(),
            active,
            bell_button_pin,
            local_address: local_address.clone(),
            broker_address: broker_address.clone(),
            broker_port,
            bell_topic: bell_topic.clone(),
            broker_user: broker_user.clone(),
            broker_password: broker_password.clone(),
        })
    }

    /// Converts a rusqlite row to a FlatEntry
    fn row_2_flat(row: &rusqlite::Row) -> Self {
        Self {
            id: row.get::<usize, u32>(0),
            name: row.get::<usize, String>(1),
            active: row.get::<usize, bool>(2),
            bell_button_pin: row.get::<usize, u8>(3),
            local_address: row.get::<usize, String>(4),
            broker_address: row.get::<usize, String>(5),
            broker_port: row.get::<usize, u16>(6),
            bell_topic: row.get::<usize, String>(7),
            broker_user: row.get::<usize, String>(8),
            broker_password: row.get::<usize, String>(9),
        }
    }

    pub fn get_all(conn: &DbConn) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw FROM flat")?;
        return stmt.query_map(&[], |row| Self::row_2_flat(&row))?.collect();
    }

    /// # Get all active flats
    ///
    /// ## Arguments
    ///
    /// * `con` - A rusqlite connection and not the rocket wrapper
    pub fn get_active(conn: &rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw FROM flat WHERE active = true")?;
        return stmt.query_map(&[], |row| Self::row_2_flat(&row))?.collect();
    }

    pub fn get_by_id(conn: &DbConn, id: u32) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw FROM flat WHERE ID=?1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[&id], |row| Self::row_2_flat(&row))?
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }

    pub fn change(&self, conn: &DbConn) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare(
            "UPDATE flat SET name = ?1, active = ?2, bell_button_pin = ?3, local_address = ?4, broker_address = ?5, broker_port = ?6, bell_topic = ?7, broker_user = ?8, broker_pw = ?9 WHERE id = ?10",
        )?;
        stmt.execute(&[
            &self.name,
            &self.active,
            &self.bell_button_pin,
            &self.local_address,
            &self.broker_address,
            &self.broker_port,
            &self.bell_topic,
            &self.id,
            &self.broker_user,
            &self.broker_password,
        ])?;
        Ok(())
    }

    pub fn delete(conn: &DbConn, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM flat WHERE id=?1 LIMIT 1", &[&id])?;
        Ok(())
    }
}
