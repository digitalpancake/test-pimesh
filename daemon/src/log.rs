use rusqlite::{Connection, Result};

pub trait Loggable {
    fn log(&self) -> Result<()>;
}

#[derive(serde::Deserialize)]
pub struct Syslog {
    id: i32,
    datetime: String,
    log_type: i32,
    data: Option<String>,
}

impl Loggable for Syslog {
    fn log(&self) -> Result<()> {
        let conn = Connection::open("/etc/pimesh/db.db")?;
        conn.execute(
            "
            INSERT INTO SysLog
            (id, datetime, log_type, data)
            VALUES (?1, ?2, ?3, ?4)",
            &[
                &self.id.to_string(),
                &self.datetime,
                &self.log_type.to_string(),
                &self.data.clone().unwrap_or("".into()),
            ],
        )?;

        Ok(())
    }
}

#[derive(serde::Deserialize)]
pub struct UserLog {
    pub id: i32,
    pub user_id: i32,
    pub datetime: String,
    pub log_type: i32,
    pub data: Option<String>,
}

impl Loggable for UserLog {
    fn log(&self) -> Result<()> {
        let conn = Connection::open("/etc/pimesh/db.db")?;
        conn.execute(
            "
            INSERT INTO UserLog
            (id, user_id, datetime, log_type, data)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &self.id.to_string(),
                &self.user_id.to_string(),
                &self.datetime,
                &self.log_type.to_string(),
                &self.data.clone().unwrap_or("".into()),
            ],
        )?;

        Ok(())
    }
}
