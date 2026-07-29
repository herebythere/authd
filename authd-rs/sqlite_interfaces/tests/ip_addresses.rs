use rusqlite::{Connection, Result};
use sqlite_interfaces::ip_addresses;
use sqlite_interfaces::ip_addresses::UpsertParams;

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
    let ip_address = match ip_addresses::upsert(
        &mut conn,
        &UpsertParams {
            organization_id: 0,
            ip_address: "127.0.0.1".to_string(),
            current_timestamp: 5,
            window_length_ms: 10,
        },
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    println!("{:?}", ip_address);

    // // read by title
    // let read_organization = match ip_addresses::read(&mut conn, "sqlite_tests") {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e.into()),
    // };

    // assert!(Some(organization.clone()) == read_organization);

    // // read by id
    // let read_organization_by_id = match ip_addresses::read_by_id(&mut conn, 0) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e.into()),
    // };

    // assert!(Some(organization.clone()) == read_organization_by_id);

    // // soft delete
    // let delete_organization = match ip_addresses::delete(
    //     &mut conn,
    //     &DeleteParams {
    //         id: 0,
    //         current_timestamp: 10,
    //     },
    // ) {
    //     Ok(ck) => ck,
    //     Err(e) => return Err(e.into()),
    // };

    // println!("{:?}, {:?}", &organization, &delete_organization);

    // match delete_organization {
    //     Some(delete_org) => {
    //         assert!(organization.id == delete_org.id);
    //         assert!(delete_org.deleted_at != None);
    //     }
    //     _ => assert!(false, "None returned after delete organization"),
    // }

    // assert!(Some(organization) == read_organization_by_id);

    // id == id
    // deleted_at == ?

    Ok(())
}
