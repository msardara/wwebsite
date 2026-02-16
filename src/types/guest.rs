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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub party_size: Option<i32>,
    pub locations: Option<Vec<String>>,
    pub default_language: Option<String>,
    pub additional_notes: Option<String>,
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
