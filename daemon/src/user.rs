use rusqlite::{Connection, Result};

use crate::db;

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub fn get_user(username: String) -> Result<()> {
    let conn = Connection::open(db::DB_PATH)?;

    let mut stmt = conn.prepare("select count(*) from User where username = ?1")?;

    let num = stmt.query([username])?;

    let _ = num.map(|_| {
        println!("Found user");
        Ok(())
    });

    Ok(())
}
