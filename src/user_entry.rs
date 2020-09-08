use crate::db_conn::{rusqlite, DbConn};
use crate::guards::GuardManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashEntry {
    pub hash: String,
    pub salt: String,
    pub config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntry {
    pub id: i64,
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
    ) -> Result<UserEntry, rusqlite::Error> {
        let hash = GuardManager::hash(&pw);

        conn.execute("INSERT INTO user (NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN) VALUES (?1, ?2, ?3, ?4, ?5)", &[name, &hash.hash, &hash.salt, &hash.config, &admin])?;
        return Ok(UserEntry {
            id: conn.last_insert_rowid(),
            name: name.clone(),
            pw_hash: hash,
            admin: admin,
        });
    }

    fn row_2_user(row: &rusqlite::Row) -> Result<UserEntry, rusqlite::Error> {
        Ok(UserEntry {
            id: row.get::<usize, i64>(0),
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
            .fold(Ok(vec![]), |mut v, r| match r {
                Ok(Ok(x)) => {
                    v.as_mut().map(|v| v.push(x));
                    v
                }
                Ok(Err(e)) => Err(e),
                Err(e) => Err(e),
            });
    }

    pub fn get_by_name(conn: DbConn, name: &String) -> Result<Vec<UserEntry>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT ID, NAME, PW_HASH, PW_SALT, PW_HASH_CONFIG, ADMIN FROM user WHERE NAME=?1",
        )?;
        return stmt
            .query_map(&[name], |row| UserEntry::row_2_user(&row))?
            .fold(Ok(vec![]), |mut v, r| match r {
                Ok(Ok(x)) => {
                    v.as_mut().map(|v| v.push(x));
                    v
                }
                Ok(Err(e)) => Err(e),
                Err(e) => Err(e),
            });
    }
}
