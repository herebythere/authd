// 0 Administrator - can add more admins or maintainers, includes Maintainer permissions
// 1 Maintainer - can update organization states


// This is ONLY for admin permissions
// Can this user delete orgs? delete users? delete sessions?

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
    pub people_id: u64,
    pub permission_kind_id: u64,
    pub deleted_at: Option<u64>,
}
