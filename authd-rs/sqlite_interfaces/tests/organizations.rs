use rusqlite::{Connection, Result};
use sqlite_interfaces::oganizations;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = oganizations::create_table(&mut conn) {
        assert!(false, "failed to create oganizations table");
    }

    // create
    let contact_kind = match oganizations::create(&mut conn, 1, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let contact_kind_read_by_id = match oganizations::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(contact_kind.clone()) == contact_kind_read_by_id);

    // read by kind
    let contact_kind_read_by_kind = match oganizations::read_by_kind(&mut conn, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(Some(contact_kind) == contact_kind_read_by_kind);

    Ok(())
}
