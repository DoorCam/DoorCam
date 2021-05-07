use super::{rusqlite, Connection, Entry, Identifier};
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./flat_test.rs"]
mod flat_test;

/// Flat entry of the corresponding "flat" table.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FlatEntry<ID: Identifier = u32> {
    pub id: ID,
    pub name: String,
    pub active: bool,
    pub bell_button_pin: u8,
    pub local_address: String,
    pub broker_address: String,
    pub broker_port: u16,
    pub bell_topic: String,
    pub broker_user: String,
    pub broker_password: String,
    pub broker_password_iv: String,
}

impl Entry for FlatEntry<u32> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare(
            "UPDATE flat SET name = ?1, active = ?2, bell_button_pin = ?3, local_address = ?4, broker_address = ?5, broker_port = ?6, bell_topic = ?7, broker_user = ?8, broker_pw = ?9, broker_pw_iv = ?10 WHERE id = ?11",
        )?;
        stmt.execute(&[
            &self.name,
            &self.active,
            &self.bell_button_pin,
            &self.local_address,
            &self.broker_address,
            &self.broker_port,
            &self.bell_topic,
            &self.broker_user,
            &self.broker_password,
            &self.broker_password_iv,
            &self.id,
        ])?;
        Ok(())
    }

    fn delete_entry(conn: &Connection, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM flat WHERE id=?1 LIMIT 1", &[&id])?;
        Ok(())
    }
}

impl FlatEntry<()> {
    pub fn create(self, conn: &Connection) -> Result<FlatEntry, rusqlite::Error> {
        conn.execute(
            "INSERT INTO flat (name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw, broker_pw_iv) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            &[
            &self.name,
            &self.active,
            &self.bell_button_pin,
            &self.local_address,
            &self.broker_address,
            &self.broker_port,
            &self.bell_topic,
            &self.broker_user,
            &self.broker_password,
            &self.broker_password_iv
            ])?;
        Ok(FlatEntry {
            id: (conn.last_insert_rowid() as u32),
            name: self.name,
            active: self.active,
            bell_button_pin: self.bell_button_pin,
            local_address: self.local_address,
            broker_address: self.broker_address,
            broker_port: self.broker_port,
            bell_topic: self.bell_topic,
            broker_user: self.broker_user,
            broker_password: self.broker_password,
            broker_password_iv: self.broker_password_iv,
        })
    }
}

impl FlatEntry<u32> {
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
            broker_password_iv: row.get::<usize, String>(10),
        }
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw, broker_pw_iv FROM flat")?;
        return stmt.query_map(&[], |row| Self::row_2_flat(&row))?.collect();
    }

    /// Get all active flats
    pub fn get_active(conn: &Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw, broker_pw_iv FROM flat WHERE active = true")?;
        return stmt.query_map(&[], |row| Self::row_2_flat(&row))?.collect();
    }

    pub fn get_by_id(conn: &Connection, id: u32) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, active, bell_button_pin, local_address, broker_address, broker_port, bell_topic, broker_user, broker_pw, broker_pw_iv FROM flat WHERE ID=?1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[&id], |row| Self::row_2_flat(&row))?
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }

    pub fn update_without_password(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut stmt = conn.prepare(
            "UPDATE flat SET name = ?1, active = ?2, bell_button_pin = ?3, local_address = ?4, broker_address = ?5, broker_port = ?6, bell_topic = ?7, broker_user = ?8 WHERE id = ?9",
        )?;
        stmt.execute(&[
            &self.name,
            &self.active,
            &self.bell_button_pin,
            &self.local_address,
            &self.broker_address,
            &self.broker_port,
            &self.bell_topic,
            &self.broker_user,
            &self.id,
        ])?;
        Ok(())
    }
}
