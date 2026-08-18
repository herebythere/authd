use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::account_requests::AccountRequest;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<AccountRequest, RusqliteError> {
    Ok(AccountRequest {
        organization_id: row.get(0)?,
        token: row.get(1)?,
        contact_kind_id: row.get(2)?,
        contact_content: row.get(3)?,
        updated_at: row.get(4)?,
        completed_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS account_requests (
            organization_id INTEGER NOT NULL,
            token INTEGER NOT NULL,
			contact_kind_id INTEGER NOT NULL,
            contact_content TEXT NOT NULL,
			updated_at INTEGER NOT NULL,
            completed_at INTEGER,
			PRIMARY KEY (contact_kind_id, contact_content)
        )",
        (),
    );

    if let Err(e) = results {
        return Err(SqliteInterfaceError::Rusqlite(e));
    }

    Ok(())
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
) -> Result<Vec<Result<AccountRequest, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            account_requests
        WHERE
            organization_id = ?1
            AND
            updated_at < ?3
            AND
			?3 - updated_at < ?2
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

    let mut entries: Vec<Result<AccountRequest, SqliteInterfaceError>> = Vec::new();
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
    pub token: i64,
    pub contact_kind_id: i64,
    pub contact_content: String,
    pub window_length_ms: i64,
    pub current_timestamp: i64,
}

pub fn increment_rate_limit(
    conn: &mut Connection,
    params: &IncrementRateLimitParams,
) -> Result<AccountRequest, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO account_requests
            (organization_id, token, contact_kind_id, contact_content, updated_at)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
		ON CONFLICT(contact_kind_id, contact_content) DO UPDATE
            SET
                token = CASE
                    WHEN
                        completed_at IS NULL AND
                        updated_at < ?5 AND
                        ?6 < (?5 - updated_at)
                        THEN ?2
                        ELSE token
                    END,
                updated_at = CASE
                    WHEN
                        completed_at IS NULL AND
                        updated_at < ?5 AND
                        ?6 < (?5 - updated_at)
                        THEN ?5
                        ELSE updated_at
                    END
            WHERE
                updated_at < ?5
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
            params.token,
            params.contact_kind_id,
            params.contact_content.clone(),
            params.current_timestamp,
            params.window_length_ms,
        ),
        get_entry_from_row,
    ) {
        Ok(entry_iter) => entry_iter,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Some(entry_maybe) = entry_iter.next() {
        println!("{:?}", entry_maybe);

        if let Ok(entry) = entry_maybe {
            return Ok(entry);
        }
    }

    Err(SqliteInterfaceError::Custom(
        "failed to rate-limit account_requests".to_string(),
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
    match conn.execute(
        "
        DELETE FROM
            account_requests
        WHERE
			organization_id = ?1
			AND
            updated_at < ?3
            AND
			?2 < (?3 - updated_at)
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
