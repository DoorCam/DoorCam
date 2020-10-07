use super::{rusqlite, DbConn};
use serde::{Deserialize, Serialize};

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
}

impl FlatEntry {
    pub fn create(
        conn: &DbConn,
        name: &String,
        active: bool,
        bell_button_pin: u8,
        local_address: &String,
        broker_address: &String,
        broker_port: u16,
        bell_topic: &String,
    ) -> Result<FlatEntry, rusqlite::Error> {
        conn.execute(
            "INSERT INTO flat (name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[name, &active, &bell_button_pin, local_address, broker_address, &broker_port, bell_topic])?;
        Ok(FlatEntry {
            id: (conn.last_insert_rowid() as u32),
            name: name.clone(),
            active,
            bell_button_pin,
            local_address: local_address.clone(),
            broker_address: broker_address.clone(),
            broker_port,
            bell_topic: bell_topic.clone(),
        })
    }

    /// Converts a rusqlite row to a FlatEntry
    fn row_2_flat(row: &rusqlite::Row) -> Result<FlatEntry, rusqlite::Error> {
        Ok(FlatEntry {
            id: row.get::<usize, u32>(0),
            name: row.get::<usize, String>(1),
            active: row.get::<usize, bool>(2),
            bell_button_pin: row.get::<usize, u8>(3),
            local_address: row.get::<usize, String>(4),
            broker_address: row.get::<usize, String>(5),
            broker_port: row.get::<usize, u16>(6),
            bell_topic: row.get::<usize, String>(7),
        })
    }

    pub fn get_all(conn: &DbConn) -> Result<Vec<FlatEntry>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic FROM flat")?;
        return stmt
            .query_map(&[], |row| FlatEntry::row_2_flat(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    /// # Get all active flats
    ///
    /// ## Arguments
    ///
    /// * `con` - A rusqlite connection and not the rocket wrapper
    pub fn get_active(conn: &rusqlite::Connection) -> Result<Vec<FlatEntry>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic FROM flat WHERE active = true")?;
        return stmt
            .query_map(&[], |row| FlatEntry::row_2_flat(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: &DbConn, id: u32) -> Result<Vec<FlatEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic FROM flat WHERE ID=?1",
        )?;
        return stmt
            .query_map(&[&id], |row| FlatEntry::row_2_flat(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn change(&self, conn: &DbConn) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE flat SET name = ?1, active = ?2, bell_button_pin = ?3, local_address = ?4, broker_address = ?5, broker_port = ?6, bell_topic = ?7 WHERE id = ?8",
            &[&self.name, &self.active, &self.bell_button_pin, &self.local_address, &self.broker_address, &self.broker_port, &self.bell_topic, &self.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &DbConn, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM flat WHERE id=?1", &[&id])?;
        Ok(())
    }
}
