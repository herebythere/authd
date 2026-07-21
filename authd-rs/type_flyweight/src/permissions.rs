// Permissions to edit:
// 0 Administrator
// 1 People
// 2 Contacts
// 3 TOTP
// 4 Sessions
// 5 Dangerous Actions

// only for people with organization=authd
// can this person from authd modify this organization

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PermissionKind {
    pub id: u64,
    pub kind: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub id: u64,
    pub role_kind_id: u64,
    pub patron_id: u64,
    pub deleted_at: Option<u64>,
}
