use rusqlite::{Connection, Result};
use sqlite_interfaces::contacts;
use sqlite_interfaces::contacts::{
    CreateParams, DangerouslyDeleteParams, DeleteParams, PatchParams,
};

use sqlite_interfaces::errors::SqliteInterfaceError;

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = contacts::create_table(&mut conn) {
        assert!(false, "failed to create contact table");
    }

    // create
    let contact = match contacts::create(
        &mut conn,
        &CreateParams {
            id: 0,
            organization_id: 1,
            people_id: 2,
            contact_kind_id: 3,
            content: "email@email.email".to_string(),
            current_timestamp: 5,
        },
    ) {
        Ok(contact) => contact,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let read_contact = match contacts::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(contact.clone()) == read_contact);

    let patch_contact = match contacts::patch(
        &mut conn,
        &PatchParams {
            id: 0,
            content: "different@email.email".to_string(),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let patched_contact = match patch_contact {
        Some(contact) => contact,
        _ => {
            return Err(SqliteInterfaceError::Custom(
                "failed to return an updated contact".to_string(),
            ))
        }
    };

    // go from some to SOME
    assert!(contact.id == patched_contact.id);
    assert!(contact.content != patched_contact.content);

    // soft delete
    let delete_contact = match contacts::delete(
        &mut conn,
        &DeleteParams {
            id: 0,
            current_timestamp: 15,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    match delete_contact {
        Some(del_contact) => {
            assert!(contact.id == del_contact.id);
            assert!(del_contact.contact_kind_id == 3);
            assert!(del_contact.content == "different@email.email");
            assert!(del_contact.deleted_at != None);
        }
        _ => assert!(false, "None returned after delete organization"),
    }

    let _ = match contacts::dangerously_delete(
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

    let read_deleted_contact = match contacts::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(read_deleted_contact == None);

    Ok(())
}
