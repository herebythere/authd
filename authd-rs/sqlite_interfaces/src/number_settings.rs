// NUMBER SETTINGS, close to contacts. no

use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::settings::NumberSettings;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<NumberSettings, RusqliteError> {
    Ok(NumberSettings {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        action_kind_id: row.get(2)?,
        number_value: row.get(3)?,
        updated_at: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS number_settings (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			action_kind_id INTEGER NOT NULL,
			number_value INTEGER NOT NULL,
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
    pub organization_id: i64,
    pub action_kind_id: bool,
    pub number_value: bool,
    pub current_timestamp: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<NumberSettings, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO number_settings
            (id, organization_id, action_kind_id, number_value, updated_at)
        VALUES
            (?1, ?2, ?3, ?4, ?5, ?6)
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
            params.action_kind_id,
            params.number_value,
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
        "failed to create number_settings".to_string(),
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
) -> Result<Vec<Result<NumberSettings, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            number_settings
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

    let mut entries: Vec<Result<NumberSettings, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub fn read_by_id(
    conn: &mut Connection,
    id: i64,
) -> Result<Option<NumberSettings>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            number_settings
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

// explicit patch (no setting null / option) (never set password_hash_results)
pub struct PatchParams {
    pub id: i64,
    pub action_kind_id: Option<bool>,
    pub number_value: Option<bool>,
    pub current_timestamp: i64,
}

pub fn patch(
    conn: &mut Connection,
    params: &PatchParams,
) -> Result<Option<NumberSettings>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE number_settings
            SET
                action_kind_id =
                    CASE
                        WHEN ?1 IS NOT NULL THEN ?1
                        ELSE action_kind_id
                    END,
                number_value =
                    CASE
                        WHEN ?2 IS NOT NULL THEN ?2
                        ELSE number_value
                    END,
				updated_at = ?3
            WHERE
                deleted_at IS NULL
                AND
                id = ?4
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(SqliteInterfaceError::Rusqlite(e));
        }
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.action_kind_id,
            params.number_value,
            params.current_timestamp,
            params.id,
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

pub struct UpdatePasswordParams {
    pub id: i64,
    pub password_hash_results: String,
    pub current_timestamp: i64,
}

// explicit isolated method to update password
pub fn update_password(
    conn: &mut Connection,
    params: &UpdatePasswordParams,
) -> Result<Option<NumberSettings>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE number_settings
            SET
                password_hash_results = ?1,
                updated_at = ?2
            WHERE
                deleted_at IS NULL
                AND
                id = ?3
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(SqliteInterfaceError::Rusqlite(e));
        }
    };

    let mut entry_iter = match stmt.query_map(
        (
            params.password_hash_results.clone(),
            params.current_timestamp,
            params.id,
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

// soft delete
pub struct DeleteParams {
    pub id: i64,
    pub current_timestamp: i64,
}

pub fn delete(
    conn: &mut Connection,
    params: &DeleteParams,
) -> Result<Option<NumberSettings>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE number_settings
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
            number_settings
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
