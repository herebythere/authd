use rusqlite::{Connection, Result};
use sqlite_interfaces::api_keys;
use sqlite_interfaces::api_keys::{CreateParams, DangerouslyDeleteParams, DeleteParams};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = api_keys::create_table(&mut conn) {
        assert!(false, "failed to create api_keys table");
    }

    // create
    let api_keys = match api_keys::create(
        &mut conn,
        &CreateParams {
            id: 0,
            organization_id: 1,
            people_id: 2,
            title: "did i do that?".to_string(),
            lifetime: 100,
        },
    ) {
        Ok(api_keys) => api_keys,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let read_api_keys = match api_keys::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(api_keys.clone()) == read_api_keys);

    // soft delete
    let delete_api_keys = match api_keys::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    match delete_api_keys {
        Some(del_api_keys) => {
            assert!(api_keys.id == del_api_keys.id);
            assert!(del_api_keys.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete organization"),
    }

    let _ = match api_keys::dangerously_delete(
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

    let read_deleted_api_keys = match api_keys::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(read_deleted_api_keys == None);

    Ok(())
}
