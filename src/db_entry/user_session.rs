use super::{rusqlite, Connection, Entry, Identifier};
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./user_session_test.rs"]
mod user_session_test;

/// User-Session entry of the corresponding "user_session" table.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserSessionEntry<ID: Identifier = u32, URef: Entry = u32> {
    pub id: ID,
    pub login_datetime: DateTime<Utc>,
    pub user: URef,
}

impl<URef: Entry> Entry for UserSessionEntry<u32, URef> {
    #[inline(always)]
    fn get_id(&self) -> u32 {
        self.id
    }

    fn delete_entry(conn: &Connection, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM user_session WHERE ID=?1", &[&id])?;
        Ok(())
    }

    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE user_session SET login_datetime = ?1, user_id = ?2 WHERE id = ?3",
            &[&self.login_datetime, &self.user.get_id(), &self.id],
        )?;
        Ok(())
    }
}

impl<URef: Entry> UserSessionEntry<(), URef> {
    pub fn create(self, conn: &Connection) -> Result<UserSessionEntry, rusqlite::Error> {
        let user_id = self.user.get_id();
        conn.execute(
            "INSERT INTO user_session (login_datetime, user_id) VALUES (?1, ?2)",
            &[&self.login_datetime, &user_id],
        )?;
        Ok(UserSessionEntry {
            id: (conn.last_insert_rowid() as u32),
            login_datetime: self.login_datetime,
            user: user_id,
        })
    }
}

impl UserSessionEntry<u32, u32> {
    /// Converts a rusqlite row to an UserEntry
    fn row_2_user(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get::<usize, u32>(0),
            login_datetime: row.get::<usize, DateTime<Utc>>(1),
            user: row.get::<usize, u32>(2),
        })
    }

    #[allow(dead_code)]
    pub fn get_all(conn: &Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT id, login_datetime, user_id FROM user_session")?;
        return stmt
            .query_map(&[], |row| Self::row_2_user(row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: &Connection, id: u32) -> Result<Option<Self>, rusqlite::Error> {
        let mut stmt = conn
            .prepare("SELECT id, login_datetime, user_id FROM user_session WHERE id=?1 LIMIT 1")?;
        return stmt
            .query_map(&[&id], |row| Self::row_2_user(row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .next()
            .map_or_else(|| Ok(None), |entry_result| entry_result.map(Some));
    }

    pub fn delete_by_user(conn: &Connection, user: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM user_session WHERE user_id=?1", &[&user])?;
        Ok(())
    }
}
