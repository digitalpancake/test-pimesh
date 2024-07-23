use rusqlite::{Connection, Result};
use std::{ops::Not, path::Path, process::exit};

pub const DB_PATH: &'static str = "/etc/pimesh/db.db";

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

    conn.execute(
        "CREATE TABLE User (
            id INTEGER PRIMARY KEY,
            username varchar(10) NOT NULL,
            password varchar(10) NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE [Group] (
            group_name varchar(10) PRIMARY KEY
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE UserGroup (
            group_name varchar(10) NOT NULL,
            user_id int NOT NULL,
            FOREIGN KEY (group_name) REFERENCES [Group](group_name),
            FOREIGN KEY (user_id) REFERENCES User(id)
        )",
        (),
    )?;

    Ok(())
}
