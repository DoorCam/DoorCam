use super::{rusqlite, DbConn, Entry, FlatEntry, Identifier, UserType};
use serde::{Deserialize, Serialize};

/// Logical entry of the hash with its parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct HashEntry {
    pub hash: String,
    pub salt: String,
    pub config: String, // TODO change to enum
}

/// User entry of the corresponding "client_user" table.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntry<ID: Identifier = u32, FRef: Entry = FlatEntry> {
    pub id: ID,
    pub name: String,
    pub pw_hash: HashEntry,
    pub user_type: UserType,
    pub active: bool,
    pub flat: Option<FRef>,
}

impl<FRef: Entry> Entry for UserEntry<u32, FRef> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn delete_entry(conn: &DbConn, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM user WHERE ID=?1 LIMIT 1", &[&id])?;
        Ok(())
    }

    fn update(&self, conn: &DbConn) -> Result<(), rusqlite::Error> {
        conn.execute(
                "UPDATE client_user SET name = ?1, pw_hash = ?2, pw_salt = ?3, pw_config = ?4, user_type = ?5, active = ?6, flat_id = ?7 WHERE id = ?8",
                &[&self.name, &self.pw_hash.hash, &self.pw_hash.salt, &self.pw_hash.config, &self.user_type, &self.active, &self.flat.as_ref().map(|flat| flat.get_id()), &self.id]
            )?;
        Ok(())
    }
}

impl<FRef: Entry> UserEntry<(), FRef> {
    pub fn create(self, conn: &DbConn) -> Result<UserEntry, rusqlite::Error> {
        let flat_id = self.flat.map(|flat| flat.get_id());
        conn.execute(
            "INSERT INTO client_user (name, pw_hash, pw_salt, pw_config, user_type, active, flat_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[&self.name, &self.pw_hash.hash, &self.pw_hash.salt, &self.pw_hash.config, &self.user_type, &self.active, &flat_id]
        )?;
        Ok(UserEntry {
            id: (conn.last_insert_rowid() as u32),
            name: self.name.clone(),
            pw_hash: self.pw_hash,
            user_type: self.user_type,
            active: self.active,
            flat: match flat_id {
                Some(flat_id) => FlatEntry::get_by_id(&conn, flat_id)?,
                None => None,
            },
        })
    }
}

impl UserEntry<u32, FlatEntry> {
    /// Converts a rusqlite row to an UserEntry
    fn row_2_user(conn: &DbConn, row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get::<usize, u32>(0),
            name: row.get::<usize, String>(1),
            pw_hash: HashEntry {
                hash: row.get::<usize, String>(2),
                salt: row.get::<usize, String>(3),
                config: row.get::<usize, String>(4),
            },
            user_type: row.get::<usize, UserType>(5),
            active: row.get::<usize, bool>(6),
            flat: match row.get::<usize, Option<u32>>(7) {
                Some(flat_id) => FlatEntry::get_by_id(&conn, flat_id)?,
                None => None,
            },
        })
    }

    pub fn get_all(conn: &DbConn) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT id, name, pw_hash, pw_salt, pw_config, user_type, active, flat_id FROM client_user")?;
        return stmt
            .query_map(&[], |row| Self::row_2_user(&conn, &row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: &DbConn, id: u32) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, pw_hash, pw_salt, pw_config, user_type, active, flat_id FROM client_user WHERE id=?1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[&id], |row| Self::row_2_user(&conn, &row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }

    #[allow(clippy::ptr_arg)]
    pub fn get_active_by_name(
        conn: &DbConn,
        name: &String,
    ) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, name, pw_hash, pw_salt, pw_config, user_type, active, flat_id FROM client_user WHERE name = ?1 AND active = 1 LIMIT 1",
        )?;
        return stmt
            .query_map(&[name], |row| Self::row_2_user(&conn, &row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }
}

impl<FRef: Entry> UserEntry<u32, FRef> {
    pub fn update_without_password(&self, conn: &DbConn) -> Result<(), rusqlite::Error> {
        conn.execute(
                "UPDATE client_user SET name = ?1, user_type = ?2, active = ?3, flat_id = ?4 WHERE id = ?5",
                &[&self.name, &self.user_type, &self.active, &self.flat.as_ref().map(|flat| flat.get_id()), &self.id],
            )?;
        Ok(())
    }

    pub fn update_unprivileged(&self, conn: &DbConn) -> Result<(), rusqlite::Error> {
        conn.execute(
                "UPDATE client_user SET name = ?1, pw_hash = ?2, pw_salt = ?3, pw_config = ?4 WHERE id = ?8",
                &[&self.name, &self.pw_hash.hash, &self.pw_hash.salt, &self.pw_hash.config, &self.id]
            )?;
        Ok(())
    }

    pub fn update_unprivileged_without_password(
        &self,
        conn: &DbConn,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE client_user SET name = ?1 WHERE id = ?5",
            &[&self.name, &self.id],
        )?;
        Ok(())
    }
}
