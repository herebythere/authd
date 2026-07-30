use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ContactKind {
    pub id: i64,
    pub title: String,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub contact_kind_id: i64,
    pub content: String,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
}
