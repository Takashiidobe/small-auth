#![allow(dead_code)]

use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rusqlite::Connection;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateUser {
    username: String,
    password: String,
    hash: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    hash: String,
}

use pwhash::bcrypt;

pub fn create_user(conn: &Connection, username: &str, password: &str) -> Result<String> {
    let username = username.to_string();
    let password = bcrypt::hash(password)?;

    let hash: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let user = CreateUser {
        username,
        password,
        hash: hash.clone(),
    };

    conn.execute(
        "INSERT INTO users (username, password, hash) VALUES (?1, ?2, ?3)",
        (&user.username, &user.password, &user.hash),
    )?;

    Ok(hash)
}

pub fn verify_user(conn: &Connection, username: &str, hash: &str) -> Result<bool> {
    let username = username.to_string();

    let mut stmt = conn.prepare("SELECT 1 from users where username = ?1 and hash = ?2;")?;

    let rows = stmt.query_map((username, hash), |row| Ok(row.get(0).unwrap()));

    if let Ok(row) = rows {
        let row: Vec<usize> = row.map(|x| x.unwrap()).collect();
        match row.as_slice() {
            [1] => Ok(true),
            _ => Ok(false),
        }
    } else {
        Ok(false)
    }
}

pub fn query_users(conn: &Connection) -> Result<Vec<Result<User, rusqlite::Error>>> {
    let mut stmt = conn.prepare("SELECT id, username, password, hash FROM users")?;
    Ok(stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0).unwrap(),
                username: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                hash: row.get(3).unwrap(),
            })
        })
        .unwrap()
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL, hash TEXT NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, UNIQUE(username), UNIQUE(hash));", []).unwrap();

        conn
    }

    #[test]
    fn create_and_verify_user() {
        let conn = setup();
        let hash = create_user(&conn, "username", "password").unwrap();
        let res = verify_user(&conn, "username", &hash).unwrap();

        assert!(res);
    }
}
