use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::organizations::Organization;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<Organization, RusqliteError> {
    Ok(Organization {
        id: row.get(0)?,
        title: row.get(1)?,
        updated_at: row.get(2)?,
        deleted_at: row.get(3)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS organizations (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
			updated_at INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err(SqliteInterfaceError::Rusqlite(e));
    }

    Ok(())
}

pub struct CreateParams {
    pub id: i64,
    pub title: String,
    pub current_timestamp: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<Organization, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO organizations
            (id, title, updated_at)
        VALUES
            (?1, ?2, ?3)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (params.id, params.title.clone(), params.current_timestamp),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(entry);
        }
    }

    Err(SqliteInterfaceError::Custom(
        "failed to create organization".to_string(),
    ))
}

pub fn read_by_id(
    conn: &mut Connection,
    id: i64,
) -> Result<Option<Organization>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            organizations
        WHERE
			deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map([id], get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

pub fn read(
    conn: &mut Connection,
    title: &str,
) -> Result<Option<Organization>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            organizations
        WHERE
            deleted_at IS NULL
            AND
            title = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map([title], get_entry_from_row) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

// update

// soft delete
pub struct DeleteParams {
    pub id: i64,
    pub current_timestamp: i64,
}

pub fn delete(
    conn: &mut Connection,
    params: &DeleteParams,
) -> Result<Option<Organization>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE organizations
            SET deleted_at = ?1
            WHERE id = ?2
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(SqliteInterfaceError::Rusqlite(e));
        }
    };

    let mut entry_iter =
        match stmt.query_map((params.current_timestamp, params.id), get_entry_from_row) {
            Ok(entry_iter) => entry_iter,
            Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
        };

    if let Some(entry_maybe) = entry_iter.next() {
        if let Ok(entry) = entry_maybe {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}
