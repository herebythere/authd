use rusqlite::{Connection, Result};
use sqlite_interfaces::action_rate_limits;
use sqlite_interfaces::action_rate_limits::{
    DangerouslyDeleteParams, IncrementRateLimitParams, ReadByPersonParams, ReadParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = action_rate_limits::create_table(&mut conn) {
        assert!(false, "failed to create action_rate_limits table");
    }

    // ratelimit
    let rate_limited_session = match action_rate_limits::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            people_id: 1,
            action_kind_id: 2,
            current_timestamp: 0,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a action_rate_limit to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(rate_limited_session.prev_window_count == 0);
    assert!(rate_limited_session.window_count == 1);

    // ratelimit again
    let rate_limited_session = match action_rate_limits::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            people_id: 1,
            action_kind_id: 2,
            current_timestamp: 5,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a action_rate_limit to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(rate_limited_session.prev_window_count == 0);
    assert!(rate_limited_session.window_count == 2);

    // read by org
    let read_sessions = match action_rate_limits::read(
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
    assert!(read_sessions.get(0) == Some(&Ok(rate_limited_session.clone())));

    // read by persons
    let read_sessions_by_person = match action_rate_limits::read_by_person(
        &mut conn,
        &ReadByPersonParams {
            organization_id: 0,
            people_id: 1,
            current_timestamp: 10,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(1 == read_sessions_by_person.len());
    assert!(read_sessions_by_person.get(0) == Some(&Ok(rate_limited_session.clone())));

    let rate_limited_session_next_window = match action_rate_limits::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            people_id: 1,
            action_kind_id: 2,
            current_timestamp: 11,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a action_rate_limit to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(rate_limited_session_next_window.window_count == 1);
    assert!(rate_limited_session_next_window.prev_window_count == 2);

    let session_new_window = match action_rate_limits::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            people_id: 1,
            action_kind_id: 2,
            current_timestamp: 32,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => match ck {
            Some(sesh) => sesh,
            _ => {
                return Err(SqliteInterfaceError::Custom(
                    "failed to find a action_rate_limit to rate-limit".to_string(),
                ))
            }
        },
        Err(e) => return Err(e),
    };

    assert!(session_new_window.window_count == 1);
    assert!(session_new_window.prev_window_count == 0);

    // dangerously delete (time delay)
    let _ = match action_rate_limits::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            organization_id: 0,
            current_timestamp: 233,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read deleted
    let read_dangerously_deleted = match action_rate_limits::read(
        &mut conn,
        &ReadParams {
            organization_id: 0,
            current_timestamp: 134,
            window_length_ms: 10,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    println!("{:?}", read_dangerously_deleted);
    assert!(0 == read_dangerously_deleted.len());

    Ok(())
}
