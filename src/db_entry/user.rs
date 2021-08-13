use super::{rusqlite, Connection, Entry, FlatEntry, Identifier, UserType};
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./user_test.rs"]
mod user_test;

/// User entry of the corresponding "client_user" table.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserEntry<ID: Identifier = u32, FRef: Entry = FlatEntry> {
    pub id: ID,
    pub name: String,
    pub password_hash: String,
    pub user_type: UserType,
    pub active: bool,
    pub flat: Option<FRef>,
}

impl<FRef: Entry> Entry for UserEntry<u32, FRef> {
    #[inline(always)]
    fn get_id(&self) -> u32 {
        self.id
    }

    fn delete_entry(conn: &Connection, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM client_user WHERE ID=?1", &[&id])?;
        Ok(())
    }

    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
                "UPDATE client_user SET name = ?1, password_hash = ?2, user_type = ?3, active = ?4, flat_id = ?5 WHERE id = ?6",
                &[&self.name, &self.password_hash, &self.user_type, &self.active, &self.flat.as_ref().map(|flat| flat.get_id()), &self.id]
            )?;
        Ok(())
    }
}

impl<FRef: Entry> UserEntry<(), FRef> {
    pub fn create(self, conn: &Connection) -> Result<UserEntry, rusqlite::Error> {
        let flat_id = self.flat.map(|flat| flat.get_id());
        conn.execute(
            "INSERT INTO client_user (name, password_hash, user_type, active, flat_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&self.name, &self.password_hash, &self.user_type, &self.active, &flat_id]
        )?;
        Ok(UserEntry {
            id: (conn.last_insert_rowid() as u32),
            name: self.name.clone(),
            password_hash: self.password_hash,
            user_type: self.user_type,
            active: self.active,
            flat: match flat_id {
                Some(flat_id) => FlatEntry::get_by_id(conn, flat_id)?,
                None => None,
            },
        })
    }
}

impl UserEntry<u32, FlatEntry> {
    /// Converts a rusqlite row to an UserEntry
    fn row_2_user(conn: &Connection, row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get::<usize, u32>(0),
            name: row.get::<usize, String>(1),
            password_hash: row.get::<usize, String>(2),
            user_type: row.get::<usize, UserType>(3),
            active: row.get::<usize, bool>(4),
            flat: match row.get::<usize, Option<u32>>(5) {
                Some(flat_id) => FlatEntry::get_by_id(conn, flat_id)?,
                None => None,
            },
        })
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, password_hash, user_type, active, flat_id FROM client_user",
        )?;
        return stmt
            .query_map(&[], |row| Self::row_2_user(conn, row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: &Connection, id: u32) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, password_hash, user_type, active, flat_id FROM client_user WHERE id=?1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[&id], |row| Self::row_2_user(conn, row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }

    #[allow(clippy::ptr_arg)]
    pub fn get_active_by_name(
        conn: &Connection,
        name: &String,
    ) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, password_hash, user_type, active, flat_id FROM client_user WHERE name = ?1 AND active = 1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[name], |row| Self::row_2_user(conn, row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }
}

impl<FRef: Entry> UserEntry<u32, FRef> {
    pub fn update_without_password(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
                "UPDATE client_user SET name = ?1, user_type = ?2, active = ?3, flat_id = ?4 WHERE id = ?5",
                &[&self.name, &self.user_type, &self.active, &self.flat.as_ref().map(|flat| flat.get_id()), &self.id],
            )?;
        Ok(())
    }

    pub fn update_unprivileged(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE client_user SET name = ?1, password_hash = ?2 WHERE id = ?3",
            &[&self.name, &self.password_hash, &self.id],
        )?;
        Ok(())
    }

    pub fn update_unprivileged_without_password(
        &self,
        conn: &Connection,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE client_user SET name = ?1 WHERE id = ?2",
            &[&self.name, &self.id],
        )?;
        Ok(())
    }
}
