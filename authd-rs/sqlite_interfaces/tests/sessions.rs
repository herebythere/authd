use rusqlite::{Connection, Result};
use sqlite_interfaces::ip_addresses;
use sqlite_interfaces::sessions;
use sqlite_interfaces::sessions::{
    CreateParams, DangerouslyDeleteParams, DeleteParams, IncrementRateLimitParams,
    ReadByPersonParams, ReadParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = sessions::create_table(&mut conn) {
        assert!(false, "failed to create ip_addresses table");
    }

    // create
    let session = match sessions::create(
        &mut conn,
        &CreateParams {
            organization_id: 0,
            people_id: 1,
            session_id: 2,
            token: 1234567,
            current_timestamp: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read by people
    let read_sessions = match sessions::read(
        &mut conn,
        &ReadParams {
            organization_id: 0,
            current_timestamp: 10,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(1 == read_sessions.len());
    assert!(read_sessions.get(0) == Some(&Ok(session.clone())));

    // read by org
    let read_sessions_by_person = match sessions::read_by_person(
        &mut conn,
        &ReadByPersonParams {
            organization_id: 0,
            people_id: 1,
            current_timestamp: 5,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(1 == read_sessions_by_person.len());
    assert!(read_sessions_by_person.get(0) == Some(&Ok(session.clone())));

    // ratelimit
    let rate_limited_session = match sessions::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            session_id: 2,
            current_timestamp: 8,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a session to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    println!("{:?}", rate_limited_session);

    assert!(rate_limited_session.window_count == session.window_count + 1);

    let rate_limited_session_next_window = match sessions::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            session_id: 2,
            current_timestamp: 16,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a session to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(rate_limited_session_next_window.window_count == 1);
    assert!(rate_limited_session_next_window.prev_window_count == 2);

    let session_new_window = match sessions::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            session_id: 2,
            current_timestamp: 37,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a session to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(session_new_window.window_count == 1);
    assert!(session_new_window.prev_window_count == 0);

    // soft delete (logout)
    let _ = match sessions::delete(
        &mut conn,
        &DeleteParams {
            session_id: 2,
            current_timestamp: 38,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    let expired_session = match sessions::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            session_id: 2,
            current_timestamp: 39,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(None == expired_session);

    // read deleted
    let read_dangerously_deleted = match sessions::read(
        &mut conn,
        &ReadParams {
            organization_id: 0,
            current_timestamp: 26,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(0 == read_dangerously_deleted.len());

    // dangerously delete (time delay)
    let _ = match sessions::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            organization_id: 0,
            current_timestamp: 139,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read deleted
    let read_dangerously_deleted = match sessions::read(
        &mut conn,
        &ReadParams {
            organization_id: 0,
            current_timestamp: 140,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(0 == read_dangerously_deleted.len());

    Ok(())
}
