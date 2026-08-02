use rusqlite::{Connection, Result};
use sqlite_interfaces::ip_addresses;
use sqlite_interfaces::ip_addresses::{
    DangerouslyDeleteParams, IncrementRateLimitParams, ReadParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

// Box<dyn std::error::Error>

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = ip_addresses::create_table(&mut conn) {
        assert!(false, "failed to create ip_addresses table");
    }

    // create
    let ip_address = match ip_addresses::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            ip_address: "127.0.0.1".to_string(),
            current_timestamp: 5,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let ip_addresses = match ip_addresses::read(
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
        Err(e) => return Err(e.into()),
    };

    assert!(1 == ip_addresses.len());
    assert!(ip_addresses.get(0) == Some(&Ok(ip_address.clone())));

    let ip_address_updated = match ip_addresses::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            ip_address: "127.0.0.1".to_string(),
            current_timestamp: 8,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(ip_address_updated.window_count == ip_address.window_count + 1);

    let ip_address_updated_again = match ip_addresses::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            ip_address: "127.0.0.1".to_string(),
            current_timestamp: 16,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(ip_address_updated_again.window_count == 1);
    assert!(ip_address_updated_again.prev_window_count == 2);

    let ip_address_new_window = match ip_addresses::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 0,
            ip_address: "127.0.0.1".to_string(),
            current_timestamp: 37,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(ip_address_new_window.window_count == 1);
    assert!(ip_address_new_window.prev_window_count == 0);

    let _ = match ip_addresses::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            organization_id: 0,
            current_timestamp: 138,
            window_length_ms: 100,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    Ok(())
}
