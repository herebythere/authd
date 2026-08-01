use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Totp {
    pub id: i64,
    pub organization_id: i64,
    pub people_id: i64,
    pub secret_key: String,
    pub algorithm: Option<i64>,
    pub period: Option<i64>,
    pub digits: Option<i64>,
    pub deleted_at: Option<i64>,
}
