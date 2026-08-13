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

		request_mfa_authentication
		
		soft_delete_staleness (permanently delete)
	
	
	ENV: secret key for public signage
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RateLimitSettings {
    pub id: i64,
	pub organization_id: i64,
	pub action_kind_id: i64,
	pub max_count: i64,
    pub window_ms: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NumberSettings {
    pub id: i64,
	pub organization_id: i64,
	pub action_kind_id: i64,
	pub number_value: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
