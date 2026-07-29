use serde::{Deserialize, Serialize};

/*
    create account
    login
    passwordless_login
    incorrect_login
    create_invite
    create_api_key
    access_api
    crawler_404
    update password
    logout



    Need ticket:
        create_contact
        create_session
        update_password
        passwordless_sign_on
        create_api_key

    Does not need ticket:
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
pub struct ActionTicketRateLimit {
    pub id: u64,
    pub people_id: Option<u64>,
    pub dangerous_action_kind_id: String,
    pub prev_window_count: u64,
    pub window_count: u64,
    pub deleted_at: Option<u64>,
}
