use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::api_keys::ApiKey;

use crate::errors::SqliteInterfaceError;

// READ BY ORG
// READ BY PEOPLE ID

fn get_entry_from_row(row: &Row) -> Result<ApiKey, RusqliteError> {
    Ok(ApiKey {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        people_id: row.get(2)?,
        title: row.get(3)?,
        lifetime: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS api_keys (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			people_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            lifetime INTEGER,
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
    pub organization_id: i64,
    pub people_id: i64,
    pub title: String,
    pub lifetime: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<ApiKey, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO api_keys
            (id, organization_id, people_id, title, lifetime)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.id,
            params.organization_id,
            params.people_id,
            params.title.clone(),
            params.lifetime,
        ),
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
        "failed to create api_keys".to_string(),
    ))
}

pub fn read_by_id(conn: &mut Connection, id: i64) -> Result<Option<ApiKey>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            api_keys
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

pub struct DeleteParams {
    pub id: i64,
    pub current_timestamp: i64,
}

pub fn delete(
    conn: &mut Connection,
    params: &DeleteParams,
) -> Result<Option<ApiKey>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE api_keys
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

pub struct DangerouslyDeleteParams {
    pub organization_id: i64,
    pub window_length_ms: i64,
    pub current_timestamp: i64,
}

pub fn dangerously_delete(
    conn: &mut Connection,
    params: &DangerouslyDeleteParams,
) -> Result<(), SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        DELETE FROM
            api_keys
        WHERE
			deleted_at IS NOT NULL
            AND
            organization_id = ?1
			AND
			?2 < (?3 - deleted_at)
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    // While deleted count != 0
    // So call delete until returned rows is 0

    let _ = match stmt.query_map(
        (
            params.organization_id,
            params.window_length_ms,
            params.current_timestamp,
        ),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    Ok(())
}
