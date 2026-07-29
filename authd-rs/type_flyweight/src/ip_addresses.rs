use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct IpAddressRateLimit {
    pub organization_id: i64,
    pub ip_address: String,
    pub prev_window_count: i64,
    pub window_count: i64,
    pub updated_at: i64,
}
