use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::contacts::Contact;

use crate::errors::SqliteInterfaceError;

// READ BY ORG
// READ BY PEOPLE ID

fn get_entry_from_row(row: &Row) -> Result<Contact, RusqliteError> {
    Ok(Contact {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        people_id: row.get(2)?,
        contact_kind_id: row.get(3)?,
        content: row.get(4)?,
        updated_at: row.get(5)?,
        deleted_at: row.get(6)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), SqliteInterfaceError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
			organization_id INTEGER NOT NULL,
			people_id INTEGER NOT NULL,
			contact_kind_id INTEGER NOT NULL,
            content TEXT NOT NULL,
			updated_at INTEGER NOT NULL,
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
    pub contact_kind_id: i64,
    pub content: String,
    pub current_timestamp: i64,
}

pub fn create(
    conn: &mut Connection,
    params: &CreateParams,
) -> Result<Contact, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO contacts
            (id, organization_id, people_id, contact_kind_id, content, updated_at)
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
            params.people_id,
            params.contact_kind_id,
            params.content.clone(),
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
        "failed to create contact".to_string(),
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
) -> Result<Vec<Result<Contact, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contacts
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

    let mut entries: Vec<Result<Contact, SqliteInterfaceError>> = Vec::new();
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
) -> Result<Vec<Result<Contact, SqliteInterfaceError>>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contacts
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

    let mut entries: Vec<Result<Contact, SqliteInterfaceError>> = Vec::new();
    for entry in entry_iter {
        match entry {
            Ok(ntry) => entries.push(Ok(ntry)),
            Err(e) => entries.push(Err(SqliteInterfaceError::Rusqlite(e))),
        }
    }

    Ok(entries)
}

pub fn read_by_id(conn: &mut Connection, id: i64) -> Result<Option<Contact>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contacts
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

// explicit patch (no setting null / option) (never set content)
pub struct PatchParams {
    pub id: i64,
    pub content: String,
    pub current_timestamp: i64,
}

pub fn patch(
    conn: &mut Connection,
    params: &PatchParams,
) -> Result<Option<Contact>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE contacts
            SET
                content = ?1,
				updated_at = ?2
            WHERE
                deleted_at IS NULL
                AND
                id = ?3
                AND
                updated_at < ?2
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
        (params.content.clone(), params.current_timestamp, params.id),
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
) -> Result<Option<Contact>, SqliteInterfaceError> {
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE contacts
            SET deleted_at = ?1
            WHERE
                id = ?2
                AND
                deleted_at IS NULL
                AND
                updated_at < ?1
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
            contacts
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
        Ok(stmt) => Ok(()),
        Err(e) => Err(SqliteInterfaceError::Rusqlite(e)),
    }
}
