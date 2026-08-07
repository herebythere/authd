use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub token: i64,
    pub prev_window_count: i64,
    pub window_count: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SessionToken {
    pub id: i64,
    pub people_id: Option<i64>,
    pub token: i64,
}
