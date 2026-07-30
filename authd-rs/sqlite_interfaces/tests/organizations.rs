use rusqlite::{Connection, Result};
use sqlite_interfaces::organizations;
use sqlite_interfaces::organizations::{
    CreateParams, DangerouslyDeleteParams, DeleteParams, PatchParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = organizations::create_table(&mut conn) {
        assert!(false, "failed to create organizations table");
    }

    // create
    let organization = match organizations::create(
        &mut conn,
        &CreateParams {
            id: 0,
            title: "sqlite_tests".to_string(),
            current_timestamp: 5,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by title
    let read_organization = match organizations::read_by_title(&mut conn, "sqlite_tests") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(organization.clone()) == read_organization);

    // read by id
    let read_organization_by_id = match organizations::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let patch_organization = match organizations::patch(
        &mut conn,
        &PatchParams {
            id: 0,
            title: "more_sqlite_tests".to_string(),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(read_organization_by_id.clone() != patch_organization);

    // soft delete
    let delete_organization = match organizations::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    match delete_organization {
        Some(delete_org) => {
            assert!(organization.id == delete_org.id);
            assert!(delete_org.title == "more_sqlite_tests");
            assert!(delete_org.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete organization"),
    }

    // dangerously delete
    let _ = match organizations::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            window_length_ms: 100,
            current_timestamp: 116,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // confirm nothing is returned
    let read_deleted_organization =
        match organizations::read_by_title(&mut conn, "more_sqlite_tests") {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(read_deleted_organization == None);

    Ok(())
}
