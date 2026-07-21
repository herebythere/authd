use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Organization {
    pub id: u64,
    pub patron_id: u64,
	pub updated_at: u64,
    pub deleted_at: Option<u64>,
}

// stretch
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PatronToOrganization {
    pub id: u64,
	pub organization_id: u64,
    pub patron_id: u64,
	pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
