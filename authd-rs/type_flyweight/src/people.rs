use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: i64,
    pub organization_id: i64,
    pub internal: bool,
    pub multi_factor_enabled: bool,
    pub password_hash_results: String,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
