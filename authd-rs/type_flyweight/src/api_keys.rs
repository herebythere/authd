use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub id: u64,
    pub organization_id: u64,
    pub people_id: u64,
    pub key: String,
    pub title: String,
    pub lifetime: u64,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApiKeyDetails {
    pub id: u64,
    pub organization_id: u64,
    pub people_id: u64,
    pub title: String,
    pub lifetime: u64,
    pub deleted_at: Option<u64>,
}
