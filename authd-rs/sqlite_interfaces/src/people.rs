use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::people::Person;

use crate::errors::SqliteInterfaceError;

fn get_entry_from_row(row: &Row) -> Result<Person, RusqliteError> {
    Ok(Person {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        internal: row.get(2)?,
        multi_factor_required: row.get(3)?,
        password_hash_results: row.get(4)?,
        updated_at: row.get(5)?,
        deleted_at: row.get(6)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			internal INTEGER NOT NULL,
			multi_factor_required INTEGER NOT NULL,
            password_hash_results TEXT NOT NULL,
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
    pub internal: bool,
    pub multi_factor_required: bool,
    pub password_hash_results: String,
    pub current_timestamp: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<Person, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO people
            (id, organization_id, internal, multi_factor_required, password_hash_results, updated_at)
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
            params.internal,
            params.multi_factor_required,
            params.password_hash_results.clone(),
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
        "failed to create people".to_string(),
    ))
}

// read by title
// paginated read
// patch

pub fn read_by_id(conn: &mut Connection, id: i64) -> Result<Option<Person>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            people
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

// update password
pub struct UpdatePasswordParams {
    pub id: i64,
    pub password_hash_results: String,
    pub current_timestamp: i64,
}

pub fn update_password(
    conn: &mut Connection,
    params: &UpdatePasswordParams,
) -> Result<Option<Person>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE people
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

// update
pub struct PatchParams {
    pub id: i64,
    pub internal: Option<bool>,
    pub multi_factor_required: Option<bool>,
    pub current_timestamp: i64,
}

pub fn patch(
    conn: &mut Connection,
    params: &PatchParams,
) -> Result<Option<Person>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE people
            SET
                internal =
                    CASE
                        WHEN ?1 IS NOT NULL THEN ?1
                        ELSE internal
                    END,
                multi_factor_required =
                    CASE
                        WHEN ?2 IS NOT NULL THEN ?2
                        ELSE multi_factor_required
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
            params.internal,
            params.multi_factor_required,
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
) -> Result<Option<Person>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE people
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

pub fn dangerously_delete_entries(
    conn: &mut Connection,
    params: &DangerouslyDeleteParams,
) -> Result<(), SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        DELETE FROM
            people
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
