use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub name: Option<String>,
    pub description: Option<String>,
    pub corresponding_world_id: Option<World>,
    pub tags: Vec<String>,
    pub session_id: String,
    pub normalized_session_id: String,
    pub host_user_id: Option<String>,
    pub host_machine_id: String,
    pub host_username: String,
    pub compatibility_hash: String,
    pub universe_id: Option<String>,
    pub neos_version: String,
    pub headless_host: bool,
    #[serde(rename = "sessionURLs")]
    pub session_urls: Vec<String>,
    pub session_users: Vec<SessionUser>,
    pub thumbnail: Option<String>,
    pub joined_users: i32,
    pub active_users: i32,
    pub max_users: i32,
    pub mobile_friendly: bool,
    #[serde(with = "crate::custom_serializer::iso_8601")]
    pub session_begin_time: DateTime<Utc>,
    #[serde(with = "crate::custom_serializer::iso_8601")]
    pub last_update: DateTime<Utc>,
    /// actually an Option<DateTime<Utc>>. but a bitch to deserialize so fuck it
    pub away_since: Option<String>,
    pub access_level: String,
    pub has_ended: bool,
    pub is_valid: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub record_id: String,
    pub owner_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub username: String,
    #[serde(rename = "userID")]
    pub user_id: Option<String>,
    pub is_present: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    pub status: i32,
    pub trace_id: String,
}
