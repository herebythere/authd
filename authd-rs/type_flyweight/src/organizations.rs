use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Organization {
    pub id: u64,
    pub title: String,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
