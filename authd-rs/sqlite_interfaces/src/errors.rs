use rusqlite::Error as RusqliteError;
use std::fmt;

#[derive(Debug)]
pub enum SqliteInterfaceError {
    Rusqlite(RusqliteError),
    Custom(String),
}

impl fmt::Display for SqliteInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SqliteInterfaceError::Rusqlite(err) => write!(f, "{}", err),
            SqliteInterfaceError::Custom(err) => write!(f, "{}", err),
        }
    }
}
