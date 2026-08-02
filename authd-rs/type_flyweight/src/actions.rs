use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionKind {
    pub id: i64,
    pub title: String,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionTicket {
    pub id: i64,
    pub organization_id: Option<i64>,
    pub people_id: Option<i64>,
    pub action_kind_id: i64,
    pub token: i64,
    pub lifetime: i64,
    pub created_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionRateLimit {
    pub id: i64,
    pub people_id: Option<i64>,
    pub action_kind_id: String,
    pub prev_window_count: i64,
    pub window_count: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
