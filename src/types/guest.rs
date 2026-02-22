//! Guest and guest-group domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::age_category::AgeCategory;
use super::dietary::DietaryPreferences;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroup {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(default)]
    pub invitation_code: String,
    pub party_size: i32,
    pub locations: Vec<String>,
    pub default_language: String,
    pub additional_notes: Option<String>,
    #[serde(default)]
    pub invitation_sent: bool,
    #[serde(default)]
    pub rsvp_submitted: bool,
    #[serde(default)]
    pub invited_by: Vec<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupWithCount {
    #[serde(flatten)]
    pub guest_group: GuestGroup,
    pub guest_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupInput {
    pub name: String,
    pub email: Option<String>,
    pub party_size: i32,
    pub locations: Vec<String>,
    pub default_language: String,
    pub additional_notes: Option<String>,
    pub invited_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_sent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Guest {
    pub id: String,
    pub guest_group_id: String,
    pub name: String,
    pub attending_locations: Vec<String>,
    pub dietary_preferences: DietaryPreferences,
    #[serde(default)]
    pub age_category: AgeCategory,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInput {
    pub guest_group_id: String,
    pub name: String,
    pub attending_locations: Vec<String>,
    pub dietary_preferences: DietaryPreferences,
    #[serde(default)]
    pub age_category: AgeCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GuestUpdate {
    pub name: Option<String>,
    pub attending_locations: Option<Vec<String>>,
    pub dietary_preferences: Option<DietaryPreferences>,
    pub age_category: Option<AgeCategory>,
}
