use rusqlite::{Connection, Result};
use sqlite_interfaces::action_kinds;
use sqlite_interfaces::action_kinds::{
    CreateParams, DangerouslyDeleteParams, DeleteParams, PatchParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = action_kinds::create_table(&mut conn) {
        assert!(false, "failed to create action_kinds table");
    }

    // create
    let action_kind = match action_kinds::create(
        &mut conn,
        &CreateParams {
            id: 0,
            title: "signups".to_string(),
            current_timestamp: 5,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read by title
    let read_action_kind = match action_kinds::read_by_title(&mut conn, "signups") {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(Some(action_kind.clone()) == read_action_kind);

    // read by id
    let read_action_kind_by_id = match action_kinds::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    // read by id
    let patch_action_kind = match action_kinds::patch(
        &mut conn,
        &PatchParams {
            id: 0,
            title: "signup".to_string(),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(read_action_kind_by_id.clone() != patch_action_kind);

    // soft delete
    let delete_action_kind = match action_kinds::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    match delete_action_kind {
        Some(delete_org) => {
            assert!(action_kind.id == delete_org.id);
            assert!(delete_org.title == "signup");
            assert!(delete_org.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete action_kind"),
    }

    // dangerously delete
    let _ = match action_kinds::dangerously_delete(
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
    let read_deleted_action_kind = match action_kinds::read_by_title(&mut conn, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e),
    };

    assert!(read_deleted_action_kind == None);

    Ok(())
}
