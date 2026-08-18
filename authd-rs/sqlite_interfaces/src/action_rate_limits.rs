use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::actions::ActionRateLimit;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<ActionRateLimit, RusqliteError> {
    Ok(ActionRateLimit {
        organization_id: row.get(0)?,
        people_id: row.get(1)?,
        action_kind_id: row.get(2)?,
        prev_window_count: row.get(3)?,
        window_count: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS action_rate_limits (
			organization_id INTEGER NOT NULL,
            people_id INTEGER NOT NULL,
            action_kind_id INTEGER NOT NULL,
			prev_window_count INTEGER NOT NULL,
			window_count INTEGER NOT NULL,
			updated_at INTEGER NOT NULL,
            PRIMARY KEY (organization_id, people_id, action_kind_id)
        )",
        (),
    ) {
        Ok(stmt) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
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
) -> Result<Vec<Result<ActionRateLimit, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            action_rate_limits
        WHERE
            organization_id = ?1
            AND
            updated_at < ?3
            AND
			(?3 - updated_at) < (?2 * 2)
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

    let mut entries: Vec<Result<ActionRateLimit, SqliteInterfaceError>> = Vec::new();
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
    pub window_length_ms: i64,
    pub current_timestamp: i64,
    pub offset: i64,
    pub limit: i64,
}

pub fn read_by_person(
    conn: &mut Connection,
    params: &ReadByPersonParams,
) -> Result<Vec<Result<ActionRateLimit, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            action_rate_limits
        WHERE
            organization_id = ?1
            AND
            people_id = ?2
            AND
            updated_at < ?4
            AND
			(?4 - updated_at) < (?3 * 2)
        LIMIT
            ?5
        OFFSET
            ?6
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let entry_iter = match stmt.query_map(
        (
            params.organization_id,
            params.people_id,
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

    let mut entries: Vec<Result<ActionRateLimit, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub struct IncrementRateLimitParams {
    pub organization_id: i64,
    pub people_id: i64,
    pub action_kind_id: i64,
    pub window_length_ms: i64,
    pub current_timestamp: i64,
}

pub fn increment_rate_limit(
    conn: &mut Connection,
    params: &IncrementRateLimitParams,
) -> Result<Option<ActionRateLimit>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO action_rate_limits
            (organization_id, people_id, action_kind_id, prev_window_count, window_count, updated_at)
        VALUES
            (?1, ?2, ?3, 0, 1, ?4)
		ON CONFLICT(organization_id, people_id, action_kind_id) DO UPDATE
            SET
                window_count =
                    CASE
                        WHEN ?5 < (?4 - updated_at) THEN 1
                        ELSE window_count + 1
                    END,
                prev_window_count =
                    CASE
                        WHEN (2 * ?5) < (?4 - updated_at) THEN 0
                        WHEN ?5 < (?4 - updated_at) THEN window_count 
                        ELSE prev_window_count
                    END,
                updated_at =
                    CASE
                        WHEN ?5 < (?4 - updated_at) THEN ?4
                        ELSE updated_at
                    END
            WHERE
                updated_at < ?4
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.organization_id,
            params.people_id,
            params.action_kind_id,
            params.current_timestamp,
            params.window_length_ms,
        ),
        get_entry_from_row,
    ) {
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
    match conn.execute(
        "
        DELETE FROM
            action_rate_limits
        WHERE
			organization_id = ?1
			AND
			(2 * ?2) < (?3 - updated_at)
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
