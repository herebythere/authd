
/*
	ratelimit (dangerous actions)
		organization kind max_count window_ms updated_at deleted_at
	
		ip_address
		api_access (web sessions)
		incorrect_login
		create_contact
		create_session
		update_password
		passwordless_sign_on
		create_api_key

	number values
		organization kind window_ms updated_at deleted_at

		origin_time_ms
		session_staleness
		passwordless_login_staleness
		reset_password_staleness
		create_contact_staleness
		
		soft_delete_staleness (permanently delete)
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RateLimitSettings {
    pub id: u64,
	pub organization_id: u64,
	pub action_kind_id: u64,
	pub max_count: u64,
    pub window_ms: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}

// "multi_factor_enabled: 1, anything but zero"
// session length
// public session length
// password min length
// password max length
// password requires numbers
// password requires letters
// password reqeuires special symbol

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NumberSettings {
    pub id: u64,
	pub organization_id: u64,
	pub action_kind_id: u64,
	pub number_value: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
