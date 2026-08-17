use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::totp::Totp;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<Totp, RusqliteError> {
    Ok(Totp {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        people_id: row.get(2)?,
        secret_key: row.get(3)?,
        algorithm: row.get(4)?,
        period: row.get(5)?,
        digits: row.get(6)?,
        deleted_at: row.get(7)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS totp (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			people_id INTEGER NOT NULL,
            secret_key TEXT NOT NULL,
            algorithm INTEGER,
			period INTEGER,
			digits INTEGER,
            deleted_at INTEGER
        )",
        (),
    ) {
        Ok(stmt) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
}

pub struct CreateParams {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub secret_key: String,
    pub algorithm: Option<i64>,
    pub period: Option<i64>,
    pub digits: Option<i64>,
}

pub fn create(conn: &mut Connection, params: &CreateParams) -> Result<Totp, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO totp
            (id, organization_id, people_id, secret_key, algorithm, period, digits)
        VALUES
            (?1, ?2, ?3, ?4, ?5, ?6, ?7)
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
            params.secret_key.clone(),
            params.algorithm,
            params.period,
            params.digits,
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
        "failed to create totp".to_string(),
    ))
}

pub struct ReadParams {
    pub organization_id: i64,
    pub offset: i64,
    pub limit: i64,
}

pub fn read(
    conn: &mut Connection,
    params: &ReadParams,
) -> Result<Vec<Result<Totp, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            totp
        WHERE
			deleted_at IS NULL
            AND
            organization_id = ?1
        LIMIT
            ?2
        OFFSET
            ?3
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let entry_iter = match stmt.query_map(
        (params.organization_id, params.limit, params.offset),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entries: Vec<Result<Totp, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub struct ReadByPersonParams {
    pub organization_id: i64,
    pub people_id: i64,
    pub offset: i64,
    pub limit: i64,
}

pub fn read_by_person(
    conn: &mut Connection,
    params: &ReadByPersonParams,
) -> Result<Vec<Result<Totp, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            totp
        WHERE
			deleted_at IS NULL
            AND
            organization_id = ?1
            AND
            people_id = ?2
        LIMIT
            ?3
        OFFSET
            ?4
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let entry_iter = match stmt.query_map(
        (
            params.organization_id,
            params.people_id,
            params.limit,
            params.offset,
        ),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entries: Vec<Result<Totp, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub fn read_by_id(conn: &mut Connection, id: i64) -> Result<Option<Totp>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            totp
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
) -> Result<Option<Totp>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE totp
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

// dangerously delete
pub fn dangerously_delete(
    conn: &mut Connection,
    params: &DangerouslyDeleteParams,
) -> Result<(), SqliteInterfaceError> {
    match conn.execute(
        "
        DELETE FROM
            totp
        WHERE
			deleted_at IS NOT NULL
            AND
			organization_id = ?1
			AND
			?2 < (?3 - deleted_at)
        ",
        (
            params.organization_id,
            params.window_length_ms,
            params.current_timestamp,
        ),
    ) {
        Ok(stmt) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
}
