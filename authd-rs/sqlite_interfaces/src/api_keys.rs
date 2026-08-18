use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::api_keys::ApiKey;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<ApiKey, RusqliteError> {
    Ok(ApiKey {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        people_id: row.get(2)?,
        title: row.get(3)?,
        token: row.get(4)?,
        lifetime: row.get(5)?,
        created_at: row.get(6)?,
        deleted_at: row.get(7)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS api_keys (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			people_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            token TEXT NOT NULL,
            lifetime INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
}

pub struct CreateParams {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub title: String,
    pub token: String,
    pub lifetime: i64,
    pub created_at: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<ApiKey, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO api_keys
            (id, organization_id, people_id, title, token, lifetime, created_at)
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
            params.title.clone(),
            params.token.clone(),
            params.lifetime,
            params.created_at,
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

pub struct ReadParams {
    pub organization_id: i64,
    pub offset: i64,
    pub limit: i64,
}

pub fn read(
    conn: &mut Connection,
    params: &ReadParams,
) -> Result<Vec<Result<ApiKey, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            api_keys
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

    let mut entries: Vec<Result<ApiKey, SqliteInterfaceError>> = Vec::new();
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
) -> Result<Vec<Result<ApiKey, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            api_keys
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

    let mut entries: Vec<Result<ApiKey, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
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
            WHERE
                deleted_at IS NULL
                AND
                id = ?2
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
            api_keys
        WHERE
			deleted_at IS NOT NULL
            AND
            organization_id = ?1
			AND
            deleted_at < ?3
            AND
			?2 < (?3 - deleted_at)
        ",
        (
            params.organization_id,
            params.window_length_ms,
            params.current_timestamp,
        ),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
}
