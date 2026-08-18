use rusqlite::{Connection, Result};
use sqlite_interfaces::account_requests;
use sqlite_interfaces::account_requests::{
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

    if let Err(_e) = account_requests::create_table(&mut conn) {
        assert!(false, "failed to create account_requets table");
    }

    let account_request = match account_requests::increment_rate_limit(
        &mut conn,
        &IncrementRateLimitParams {
            organization_id: 2,
            token: 1234567,
            contact_kind_id: 4,
            contact_content: "we're normal men".to_string(),
            window_length_ms: 10,
            current_timestamp: 5,
        },
    ) {
        Ok(arqst) => arqst,
        Err(e) => return Err(e),
    };

    // let account_request_again = match account_requests::increment_rate_limit(
    //     &mut conn,
    //     &IncrementRateLimitParams {
    //         organization_id: 2,
    //         token: 1234567,
    //         contact_kind_id: 4,
    //         content: "we're normal men".to_string(),
    //         window_length_ms: 10,
    //         current_timestamp: 6,
    //     },
    // ) {
    //     Ok(account_request) => account_request,
    //     Err(e) => return Err(e),
    // };

    // let account_request_new_window = match account_requests::increment_rate_limit(
    //     &mut conn,
    //     &IncrementRateLimitParams {
    //         organization_id: 2,
    //         token: 1234567,
    //         contact_kind_id: 4,
    //         content: "we're normal men".to_string(),
    //         window_length_ms: 10,
    //         current_timestamp: 17,
    //     },
    // ) {
    //     Ok(account_request) => account_request,
    //     Err(e) => return Err(e),
    // };

    // // read
    // let read_account_requests = match account_requests::read(
    //     &mut conn,
    //     &ReadParams {
    //         organization_id: 2,
    //         window_length_ms: 10,
    //         current_timestamp: 17,
    //         limit: 16,
    //         offset: 0,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    // assert!(1 == read_account_requests.len());
    // assert!(read_account_requests.get(0) == Some(&Ok(account_request.clone())));

    // let _ = match account_requests::dangerously_delete(
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

    // let read_deleted_people = match account_requests::read(
    //     &mut conn,
    //     &ReadParams {
    //         organization_id: 2,
    //         window_length_ms: 10,
    //         current_timestamp: 17,
    //         limit: 16,
    //         offset: 0,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e),
    // };

    // assert!(1 == read_deleted_people.len());

    Ok(())
}
