use rusqlite::{Connection, Result};
use sqlite_interfaces::ip_addresses;
use sqlite_interfaces::sessions;
use sqlite_interfaces::sessions::{
    CreateParams, DangerouslyDeleteParams, IncrementRateLimitParams, ReadParams,
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
            session_id: 0,
            token: 1234567,
            current_timestamp: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    let read_session = match sessions::read(
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

    // ratelimit
    let rate_limited_session = match sessions::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            session_id: 1234567,
            current_timestamp: 5,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // assert!(1 == ip_addresses.len());
    // assert!(ip_addresses.get(0) == Some(&Ok(ip_address.clone())));

    // let ip_address_updated = match sessions::increment_rate_limit(
    //     &mut conn,
    //     &IncrementRateLimitParams {
    //         organization_id: 0,
    //         ip_address: "127.0.0.1".to_string(),
    //         current_timestamp: 8,
    //         window_length_ms: 10,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    // assert!(ip_address_updated.window_count == ip_address.window_count + 1);

    // let ip_address_updated_again = match sessions::increment_rate_limit(
    //     &mut conn,
    //     &IncrementRateLimitParams {
    //         organization_id: 0,
    //         ip_address: "127.0.0.1".to_string(),
    //         current_timestamp: 16,
    //         window_length_ms: 10,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    // assert!(ip_address_updated_again.window_count == 1);
    // assert!(ip_address_updated_again.prev_window_count == 2);

    // let ip_address_new_window = match sessions::increment_rate_limit(
    //     &mut conn,
    //     &IncrementRateLimitParams {
    //         organization_id: 0,
    //         ip_address: "127.0.0.1".to_string(),
    //         current_timestamp: 37,
    //         window_length_ms: 10,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    // assert!(ip_address_new_window.window_count == 1);
    // assert!(ip_address_new_window.prev_window_count == 0);

    // let _ = match sessions::dangerously_delete(
    //     &mut conn,
    //     &DangerouslyDeleteParams {
    //         organization_id: 0,
    //         current_timestamp: 138,
    //         window_length_ms: 100,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    Ok(())
}
