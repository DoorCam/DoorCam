use super::{rusqlite, DbConn, UserType};
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
    pub user_type: UserType,
    pub active: bool,
    pub flat_id: Option<u32>,
    pub flat_name: Option<String>,
}

impl UserEntry {
    pub fn create(
        conn: &DbConn,
        name: &String,
        pw: &String,
        user_type: UserType,
        active: bool,
        flat_id: Option<u32>,
    ) -> Result<UserEntry, AuthError> {
        GuardManager::check_password(pw)?;
        let hash = GuardManager::hash(&pw);

        conn.execute(
            "INSERT INTO client_user (name, pw_hash, pw_salt, pw_config, user_type, active, flat_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[name, &hash.hash, &hash.salt, &hash.config, &user_type, &active, &flat_id]
        )?;
        return Ok(UserEntry {
            id: (conn.last_insert_rowid() as u32),
            name: name.clone(),
            pw_hash: hash,
            user_type: user_type,
            active: active,
            flat_id: flat_id,
            flat_name: None,
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
            user_type: row.get::<usize, UserType>(5),
            active: row.get::<usize, bool>(6),
            flat_id: row.get::<usize, Option<u32>>(7),
            flat_name: row.get::<usize, Option<String>>(8),
        })
    }

    pub fn get_all(conn: &DbConn) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt =
            conn.prepare("SELECT c.id, c.name, c.pw_hash, c.pw_salt, c.pw_config, c.user_type, c.active, c.flat_id, f.name FROM client_user AS c LEFT JOIN flat AS f ON c.flat_id = f.id")?;
        return stmt
            .query_map(&[], |row| UserEntry::row_2_user(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_by_id(conn: &DbConn, id: u32) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT c.id, c.name, c.pw_hash, c.pw_salt, c.pw_config, c.user_type, c.active, c.flat_id, f.name FROM client_user AS c LEFT JOIN flat AS f ON c.flat_id = f.id WHERE c.id=?1",
        )?;
        return stmt
            .query_map(&[&id], |row| UserEntry::row_2_user(&row))?
            .map(|r| match r {
                Ok(x) => x,
                Err(e) => Err(e),
            })
            .collect();
    }

    pub fn get_active_by_name(
        conn: &DbConn,
        name: &String,
    ) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT c.id, c.name, c.pw_hash, c.pw_salt, c.pw_config, c.user_type, c.active, c.flat_id, f.name FROM client_user AS c LEFT JOIN flat AS f ON c.flat_id = f.id WHERE c.name = ?1 AND c.active = 1",
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
        conn: &DbConn,
        id: u32,
        name: &String,
        pw: &String,
        user_type: UserType,
        active: bool,
        flat_id: Option<u32>,
    ) -> Result<(), AuthError> {
        if pw.is_empty() {
            conn.execute(
                "UPDATE client_user SET name = ?1, user_type = ?2, active = ?3, flat_id = ?4 WHERE id = ?5",
                &[name, &user_type, &active, &flat_id, &id],
            )?;
        } else {
            GuardManager::check_password(pw)?;
            let hash = GuardManager::hash(&pw);

            conn.execute(
                "UPDATE client_user SET name = ?1, pw_hash = ?2, pw_salt = ?3, pw_config = ?4, user_type = ?5, active = ?6, flat_id = ?7 WHERE id = ?8",
                &[name, &hash.hash, &hash.salt, &hash.config, &user_type, &active, &flat_id, &id]
            )?;
        }
        return Ok(());
    }

    pub fn delete(conn: &DbConn, id: u32) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM user WHERE ID=?1", &[&id])?;
        return Ok(());
    }
}
