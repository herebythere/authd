use rusqlite::{Connection, Result};
use sqlite_interfaces::people;
use sqlite_interfaces::people::{CreateParams, DeleteParams, PatchParams, UpdatePasswordParams};

use sqlite_interfaces::errors::SqliteInterfaceError;

// Box<dyn std::error::Error>

#[test]
fn crud_operations() -> Result<(), SqliteInterfaceError> {
    let mut conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(e) => return Err(SqliteInterfaceError::Rusqlite(e)),
    };

    if let Err(_e) = people::create_table(&mut conn) {
        assert!(false, "failed to create people table");
    }

    // // create
    let person = match people::create(
        &mut conn,
        &CreateParams {
            id: 0,
            organization_id: 2,
            internal: true,
            multi_factor_enabled: true,
            password_hash_results: "im just a little guy".to_string(),
            current_timestamp: 5,
        },
    ) {
        Ok(person) => person,
        Err(e) => return Err(e.into()),
    };

    // // read by id
    let read_person = match people::read_by_id(&mut conn, 0) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(person.clone()) == read_person);

    // update passwords
    let update_passwords = match people::update_password(
        &mut conn,
        &UpdatePasswordParams {
            id: 0,
            password_hash_results: "uncle iroh is dissapointed".to_string(),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(person.clone()) != update_passwords);

    let patch_person = match people::patch(
        &mut conn,
        &PatchParams {
            id: 0,
            internal: None,
            multi_factor_enabled: Some(false),
            current_timestamp: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let patched_person = match patch_person {
        Some(person) => person,
        _ => {
            return Err(SqliteInterfaceError::Custom(
                "failed to return an updated person".to_string(),
            ))
        }
    };

    // go from some to SOME
    assert!(person.id == patched_person.id);
    assert!(person.password_hash_results != patched_person.password_hash_results);

    // // soft delete
    // let delete_organization = match organizations::delete(
    //     &mut conn,
    //     &DeleteParams {
    //         id: 0,
    //         current_timestamp: 15,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e.into()),
    // };

    // match delete_organization {
    //     Some(delete_org) => {
    //         assert!(organization.id == delete_org.id);
    //         assert!(delete_org.title == "more_sqlite_tests");
    //         assert!(delete_org.deleted_at != None);
    //     }
    //     _ => assert!(false, "None returned after delete organization"),
    // }

    Ok(())
}
