use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::ip_addresses::IpAddressRateLimit;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<IpAddressRateLimit, RusqliteError> {
    Ok(IpAddressRateLimit {
        organization_id: row.get(0)?,
        ip_address: row.get(1)?,
        prev_window_count: row.get(2)?,
        window_count: row.get(3)?,
        updated_at: row.get(4)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS ip_addresses (
            organization_id INTEGER NOT NULL,
            ip_address TEXT NOT NULL,
			prev_window_count INTEGER NOT NULL,
			window_count INTEGER NOT NULL,
			updated_at INTEGER NOT NULL,
			PRIMARY KEY (organization_id, ip_address)
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
) -> Result<Vec<Result<IpAddressRateLimit, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            ip_addresses
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

    let mut entries: Vec<Result<IpAddressRateLimit, SqliteInterfaceError>> = Vec::new();
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
    pub window_length_ms: i64,
    pub ip_address: String,
    pub current_timestamp: i64,
}

// INSERT INTO users(username,score) VALUES('Johnny', 388)
// ON CONFLICT(username) DO UPDATE SET score = '388';
pub fn increment_rate_limit(
    conn: &mut Connection,
    params: &IncrementRateLimitParams,
) -> Result<IpAddressRateLimit, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO ip_addresses
            (organization_id, ip_address, prev_window_count, window_count, updated_at)
        VALUES
            (?1, ?2, 0, 1, ?3)
		ON CONFLICT(organization_id, ip_address) DO UPDATE
            SET
                window_count =
                    CASE
                        WHEN ?4 < (?3 - updated_at) THEN 1
                        ELSE window_count + 1
                    END,
                prev_window_count =
                    CASE
                        WHEN (2 * ?4) < (?3 - updated_at) THEN 0
                        WHEN ?4 < (?3 - updated_at) THEN window_count 
                        ELSE prev_window_count
                    END,
                updated_at =
                    CASE
                        WHEN ?4 < (?3 - updated_at) THEN ?3
                        ELSE updated_at
                    END
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
            params.ip_address.clone(),
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
            return Ok(entry);
        }
    }

    Err(SqliteInterfaceError::Custom(
        "failed to rate-limit ip address".to_string(),
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
            ip_addresses
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
