use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Organization {
    pub id: i64,
    pub title: String,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
