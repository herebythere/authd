use serde::{Deserialize, Serialize};

/*
    Need token:
        create_contact
        create_session
        update_password
        passwordless_sign_on
        create_api_key

    Does not need token:
        login rate limit
        ip_address rate limit
        api_access rate limit
        incorrect_login rate limit
        create_api_key rate limit
*/

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionKind {
    pub id: u64,
    pub kind: String,
    pub requires_ticket: bool,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionTicket {
    pub id: u64,
    pub action_kind_id: u64,
    pub organization_id: Option<u64>,
    pub people_id: Option<u64>,
    pub token: u64,
    pub contact_kind_id: u64,
    pub contact_content: String,
    pub deleted_at: Option<u64>,
}

// dangerous rate limit?
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DangerousActionRateLimit {
    pub id: u64,
    pub people_id: Option<u64>,
    pub dangerous_action_kind_id: String,
    pub prev_window_count: u64,
    pub window_count: u64,
    pub deleted_at: Option<u64>,
}
