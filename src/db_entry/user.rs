use super::{rusqlite, DbConn};
use crate::guards::{AuthError, GuardManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashEntry {
    pub hash: String,
    pub salt: String,
    pub config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntry {
    pub id: u32,
    pub name: String,
    pub pw_hash: HashEntry,
    pub admin: bool,
}

impl UserEntry {
    pub fn create(
        conn: DbConn,
        name: &String,
        pw: &String,
        admin: bool,
    ) -> Result<UserEntry, AuthError> {
        GuardManager::check_password(pw)?;
        let hash = GuardManager::hash(&pw);

        conn.execute("INSERT INTO user (NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN) VALUES (?1, ?2, ?3, ?4, ?5)", &[name, &hash.hash, &hash.salt, &hash.config, &admin])?;
        return Ok(UserEntry {
            id: (conn.last_insert_rowid() as u32),
            name: name.clone(),
            pw_hash: hash,
            admin: admin,
        });
    }

    fn row_2_user(row: &rusqlite::Row) -> Result<UserEntry, rusqlite::Error> {
        Ok(UserEntry {
            id: row.get::<usize, u32>(0),
            name: row.get::<usize, String>(1),
            pw_hash: HashEntry {
                hash: row.get::<usize, String>(2),
                salt: row.get::<usize, String>(3),
                config: row.get::<usize, String>(4),
            },
            admin: row.get::<usize, bool>(5),
        })
    }

    pub fn get_all(conn: DbConn) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT ID, NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN FROM user")?;
        return stmt
            .query_map(&[], |row| UserEntry::row_2_user(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: DbConn, id: u32) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT ID, NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN FROM user WHERE ID=?1",
        )?;
        return stmt
            .query_map(&[&id], |row| UserEntry::row_2_user(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_name(conn: DbConn, name: &String) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT ID, NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN FROM user WHERE NAME=?1",
        )?;
        return stmt
            .query_map(&[name], |row| UserEntry::row_2_user(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn change(
        conn: DbConn,
        id: u32,
        name: &String,
        pw: &String,
        admin: bool,
    ) -> Result<(), AuthError> {
        if pw.is_empty() {
            conn.execute(
                "UPDATE user SET NAME = ?1, ADMIN = ?2 WHERE ID = ?3",
                &[name, &admin, &id],
            )?;
        } else {
            GuardManager::check_password(pw)?;
            let hash = GuardManager::hash(&pw);

            conn.execute(
                "UPDATE user SET NAME = ?1, PW_HASH = ?2, PW_SALT = ?3, PW_HASH_CONFIG = ?4, ADMIN = ?5 WHERE ID = ?6",
                &[name, &hash.hash, &hash.salt, &hash.config, &admin, &id]
            )?;
        }
        return Ok(());
    }

    pub fn delete(conn: DbConn, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM user WHERE ID=?1", &[&id])?;
        return Ok(());
    }
}
