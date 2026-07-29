use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: u64,
    pub organization_id: u64,
    pub internal: bool,
    pub multi_factor_enabled: bool,
    pub password_hash_results: String,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
