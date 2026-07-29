use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct IpAddressRateLimit {
    pub ip_address: String,
    pub organization_id: u64,
    pub prev_window_count: u64,
    pub window_count: u64,
    pub updated_at: u64,
}
