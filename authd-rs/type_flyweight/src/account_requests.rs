use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AccountRequest {
    pub organization_id: i64,
    pub token: i64,
    pub contact_kind_id: i64,
    pub contact_content: String,
    pub updated_at: i64,
    pub completed_at: i64,
}
