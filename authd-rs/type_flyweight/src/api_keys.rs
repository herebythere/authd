use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub title: String,
    pub lifetime: i64,
    pub deleted_at: Option<i64>,
}
