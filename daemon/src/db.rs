use rusqlite::{Connection, Result};
use std::{ops::Not, path::Path, process::exit};

const DB_PATH: &'static str = "/etc/pimesh/db.db";

pub fn create_db() -> Result<()> {
    Path::new("/etc/pimesh")
        .exists()
        .not()
        .then(|| match std::fs::create_dir_all("/etc/pimesh") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        });

    let _ = std::fs::remove_file(DB_PATH);
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE User (
            id INTEGER PRIMARY KEY,
            creation_datetime DateTime
            username varchar(30) NOT NULL,
            password varchar(30) NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE UserLog (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            datetime DateTime NOT NULL,
            log_type INT NOT NULL,
            data varchar(10)
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE SysLog (
            id INTEGER PRIMARY KEY,
            datetime DateTime NOT NULL,
            log_type INT NOT NULL,
            data varchar(10)
        )",
        (),
    )?;

    Ok(())
}
