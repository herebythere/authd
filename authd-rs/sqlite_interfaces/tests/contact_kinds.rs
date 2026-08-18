use rusqlite::{Connection, Result};
use sqlite_interfaces::contact_kinds;
use sqlite_interfaces::contact_kinds::{
    CreateParams, DangerouslyDeleteParams, DeleteParams, PatchParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = contact_kinds::create_table(&mut conn) {
        assert!(false, "failed to create contact_kinds table");
    }

    // create
    let contact_kind = match contact_kinds::create(
        &mut conn,
        &CreateParams {
            id: 0,
            title: "emails".to_string(),
            current_timestamp: 5,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read by title
    let read_contact_kind = match contact_kinds::read_by_title(&mut conn, "emails") {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(Some(contact_kind.clone()) == read_contact_kind);

    // read by id
    let read_contact_kind_by_id = match contact_kinds::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read by id
    let patch_contact_kind = match contact_kinds::patch(
        &mut conn,
        &PatchParams {
            id: 0,
            title: "email".to_string(),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(read_contact_kind_by_id.clone() != patch_contact_kind);

    // soft delete
    let delete_contact_kind = match contact_kinds::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    match delete_contact_kind {
        Some(delete_org) => {
            assert!(contact_kind.id == delete_org.id);
            assert!(delete_org.title == "email");
            assert!(delete_org.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete contact_kind"),
    }

    // dangerously delete
    let _ = match contact_kinds::dangerously_delete(
        &mut conn,
        &DangerouslyDeleteParams {
            window_length_ms: 100,
            current_timestamp: 116,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // confirm nothing is returned
    let read_deleted_contact_kind = match contact_kinds::read_by_title(&mut conn, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(read_deleted_contact_kind == None);

    Ok(())
}
