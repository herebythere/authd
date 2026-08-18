use rusqlite::{Connection, Result};
use sqlite_interfaces::totp;
use sqlite_interfaces::totp::{CreateParams, DangerouslyDeleteParams, DeleteParams, ReadParams};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = totp::create_table(&mut conn) {
        assert!(false, "failed to create totp table");
    }

    // create
    let totp = match totp::create(
        &mut conn,
        &CreateParams {
            id: 0,
            organization_id: 1,
            people_id: 2,
            secret_key: "look what i can do".to_string(),
            algorithm: None,
            period: None,
            digits: None,
        },
    ) {
        Ok(totp) => totp,
        Err(e) => return Err(e),
    };

    // read by id
    let read_totp = match totp::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(Some(totp.clone()) == read_totp);

    // read
    let read_totps = match totp::read(
        &mut conn,
        &ReadParams {
            organization_id: 1,
            limit: 16,
            offset: 0,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(1 == read_totps.len());
    assert!(read_totps.get(0) == Some(&Ok(totp.clone())));

    // soft delete
    let delete_totp = match totp::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    match delete_totp {
        Some(del_totp) => {
            assert!(totp.id == del_totp.id);
            assert!(del_totp.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete totp"),
    }

    let _ = match totp::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            organization_id: 0,
            current_timestamp: 138,
            window_length_ms: 100,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    let read_deleted_totp = match totp::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(read_deleted_totp == None);

    Ok(())
}
