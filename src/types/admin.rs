//! Admin-specific types (dashboard statistics, RSVP status).

use serde::{Deserialize, Serialize};

use super::guest::GuestGroup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminStats {
    /// Total number of invited guests (count of actual guests in the guests table)
    pub total_guests: i32,
    /// Total number of guests who confirmed attendance (guests with location selections)
    pub total_confirmed: i32,
    /// Total number of pending guests (guests with no location selections)
    pub pending_rsvps: i32,
    /// Total guests attending Sardinia
    pub sardinia_guests: i32,
    /// Total guests attending Tunisia
    pub tunisia_guests: i32,
    /// Total guests invited to both locations
    pub both_locations_guests: i32,
    pub vegetarian_count: i32,
    pub vegan_count: i32,
    pub halal_count: i32,
    pub no_pork_count: i32,
    pub gluten_free_count: i32,
    pub other_dietary_count: i32,
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestGroupWithRsvp {
    #[serde(flatten)]
    pub guest_group: GuestGroup,
    pub rsvp_status: RsvpStatus,
    pub total_attending: i32,
}
