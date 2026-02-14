use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Sardinia,
    Tunisia,
    Both,
}

#[allow(dead_code)]
impl Location {
    pub fn as_str(&self) -> &str {
        match self {
            Location::Sardinia => "sardinia",
            Location::Tunisia => "tunisia",
            Location::Both => "both",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sardinia" => Some(Location::Sardinia),
            "tunisia" => Some(Location::Tunisia),
            "both" => Some(Location::Both),
            _ => None,
        }
    }

    pub fn includes(&self, other: &Location) -> bool {
        match (self, other) {
            (Location::Both, _) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroup {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub invitation_code: String,
    pub party_size: i32,
    pub location: Location,
    pub default_language: String,
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
    pub invitation_code: String,
    pub party_size: i32,
    pub location: String,
    pub default_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub party_size: Option<i32>,
    pub location: Option<String>,
    pub default_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rsvp {
    pub id: String,
    pub guest_group_id: String,
    pub location: String, // 'sardinia' or 'tunisia'
    pub attending: bool,
    pub number_of_guests: i32,
    pub additional_notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsvpWithDietaryCounts {
    #[serde(flatten)]
    pub rsvp: Rsvp,
    pub dietary_vegetarian: i32,
    pub dietary_vegan: i32,
    pub dietary_halal: i32,
    pub dietary_no_pork: i32,
    pub dietary_gluten_free: i32,
    pub dietary_other: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsvpInput {
    pub guest_group_id: String,
    pub location: String, // 'sardinia' or 'tunisia'
    pub attending: bool,
    pub number_of_guests: i32,
    pub additional_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DietaryPreferences {
    pub vegetarian: bool,
    pub vegan: bool,
    pub halal: bool,
    pub no_pork: bool,
    pub gluten_free: bool,
    pub other: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Guest {
    pub id: String,
    pub guest_group_id: String,
    pub name: String,
    pub dietary_preferences: DietaryPreferences,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestInput {
    pub guest_group_id: String,
    pub name: String,
    pub dietary_preferences: DietaryPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GuestUpdate {
    pub name: Option<String>,
    pub dietary_preferences: Option<DietaryPreferences>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: String,
    pub key: String,
    pub language: String,
    pub location: Option<Location>,
    pub value: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub filename: String,
    pub caption: Option<String>,
    pub display_order: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoInput {
    pub filename: String,
    pub caption: Option<String>,
    pub display_order: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub key: String,
    pub value: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminStats {
    /// Total number of invited guests (count of actual guests in the guests table)
    pub total_guests: i32,
    /// Total number of guests who confirmed attendance (sum of number_of_guests from attending RSVPs)
    pub total_confirmed: i32,
    /// Total number of pending guests (count of guests from groups that haven't RSVP'd)
    pub pending_rsvps: i32,
    /// Total guests invited to Sardinia (count from guests table, includes guests attending Both locations)
    pub sardinia_guests: i32,
    /// Total guests invited to Tunisia (count from guests table, includes guests attending Both locations)
    pub tunisia_guests: i32,
    /// Total number of pending guest group invitations (groups that haven't RSVP'd)
    pub both_locations_guests: i32,
    pub vegetarian_count: i32,
    pub vegan_count: i32,
    pub halal_count: i32,
    pub no_pork_count: i32,
    pub gluten_free_count: i32,
    pub other_dietary_count: i32,
    /// Total attending (same as total_confirmed)
    pub total_attending: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupWithRsvp {
    #[serde(flatten)]
    pub guest_group: GuestGroup,
    pub rsvp_status: RsvpStatus,
    pub total_attending: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RsvpStatus {
    Pending,
    Confirmed,
    Declined,
    Partial,
}

impl RsvpStatus {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            RsvpStatus::Pending => "Pending",
            RsvpStatus::Confirmed => "Confirmed",
            RsvpStatus::Declined => "Declined",
            RsvpStatus::Partial => "Partial",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    French,
    Italian,
}

#[allow(dead_code)]
impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Italian => "it",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "fr" => Language::French,
            "it" => Language::Italian,
            _ => Language::English,
        }
    }

    pub fn from_browser() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(navigator) = window.navigator().language() {
                    let lang = navigator.to_lowercase();
                    if lang.starts_with("fr") {
                        return Language::French;
                    } else if lang.starts_with("it") {
                        return Language::Italian;
                    }
                }
            }
        }
        Language::English
    }

    pub fn name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Italian => "Italiano",
        }
    }
}

// ============================================================================
// Supabase Auth Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: Option<String>,
    pub user_metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub user: AuthUser,
}
