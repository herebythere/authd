use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::sessions::Session;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<Session, RusqliteError> {
    Ok(Session {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        people_id: row.get(2)?,
        token: row.get(3)?,
        prev_window_count: row.get(4)?,
        window_count: row.get(5)?,
        updated_at: row.get(6)?,
        deleted_at: row.get(7)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
            people_id INTEGER NOT NULL,
            token INTEGER NOT NULL,
			prev_window_count INTEGER NOT NULL,
			window_count INTEGER NOT NULL,
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
    pub session_id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub token: i64,
    pub current_timestamp: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<Session, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO sessions
            (id, organization_id, people_id, token, prev_window_count, window_count, updated_at)
        VALUES
            (?1, ?2, ?3, ?4, 0, 1, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.session_id,
            params.organization_id,
            params.people_id,
            params.token,
            params.current_timestamp,
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
        "failed to create session".to_string(),
    ))
}

pub struct ReadParams {
    pub organization_id: i64,
    pub window_length_ms: i64,
    pub current_timestamp: i64,
    pub offset: i64,
    pub limit: i64,
}

pub fn read(
    conn: &mut Connection,
    params: &ReadParams,
) -> Result<Vec<Result<Session, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            sessions
        WHERE
            organization_id = ?1
            AND
			?2 * 2 < ?3 - updated_at 
        LIMIT
            ?4
        OFFSET
            ?5
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let entry_iter = match stmt.query_map(
        (
            params.organization_id,
            params.window_length_ms,
            params.current_timestamp,
            params.limit,
            params.offset,
        ),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entries: Vec<Result<Session, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub struct IncrementRateLimitParams {
    pub window_length_ms: i64,
    pub session_id: i64,
    pub current_timestamp: i64,
}

pub fn increment_rate_limit(
    conn: &mut Connection,
    params: &IncrementRateLimitParams,
) -> Result<Session, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE sessions
            SET
                window_count =
                    CASE
                        WHEN ?3 < (?2 - updated_at) THEN 1
                        ELSE window_count + 1
                    END,
                prev_window_count =
                    CASE
                        WHEN (2 * ?3) < (?2 - updated_at) THEN 0
                        WHEN ?3 < (?2 - updated_at) THEN window_count 
                        ELSE prev_window_count
                    END,
                updated_at =
                    CASE
                        WHEN ?3 < (?2 - updated_at) THEN ?2
                        ELSE updated_at
                    END
			WHERE
				id = ?1
                AND
                deleted_at IS NULL
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.session_id,
            params.current_timestamp,
            params.window_length_ms,
        ),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        println!("{:?}", &entry_maybe);

        if let Ok(entry) = entry_maybe {
            println!("{:?}", &entry);

            return Ok(entry);
        }
    }

    Err(SqliteInterfaceError::Custom(
        "failed to rate-limit session".to_string(),
    ))
}

// Soft Delete
pub struct DeleteParams {
    pub session_id: i64,
    pub current_timestamp: i64,
}

pub fn delete(
    conn: &mut Connection,
    params: &DeleteParams,
) -> Result<Session, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE sessions
            SET
                deleted_at = ?2
			WHERE
				id = ?1
                AND
                deleted_at IS NULL
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (params.session_id, params.current_timestamp),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        println!("{:?}", &entry_maybe);

        if let Ok(entry) = entry_maybe {
            println!("{:?}", &entry);

            return Ok(entry);
        }
    }

    Err(SqliteInterfaceError::Custom(
        "failed to rate-limit session".to_string(),
    ))
}

// paginated read
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
    let mut stmt = match conn.prepare(
        "
        DELETE FROM
            sessions
        WHERE
			organization_id = ?1
			AND
			(?2 * 2) < (?3 - updated_at)
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

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
