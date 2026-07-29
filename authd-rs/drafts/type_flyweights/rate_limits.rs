/*
	org_id	user_id	category	prev_window_count	window_count
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RateLimit {
    pub id: u64,
	pub organization_id: u64,
    pub people_id: u64,
	pub action_kind_id: u64,
	pub prev_window_count: u64,
    pub window_count: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
