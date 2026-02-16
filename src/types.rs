//! Domain types for the wedding website.
//!
//! Types are split into focused submodules for easier maintenance:
//!
//! - [`age_category`] — `AgeCategory` enum
//! - [`admin`] — `AdminStats`, `RsvpStatus`, `GuestGroupWithRsvp`
//! - [`auth`] — Supabase auth types (`AuthSession`, `AuthUser`, etc.)
//! - [`dietary`] — `DietaryPreferences` with display/badge helpers
//! - [`guest`] — `GuestGroup`, `Guest`, input/update structs
//! - [`language`] — `Language` enum with browser detection
//! - [`location`] — `Location` enum with display metadata

mod admin;
mod age_category;
mod auth;
mod dietary;
mod guest;
mod language;
mod location;

// Re-export everything so that `use crate::types::*` continues to work.
#[allow(unused_imports)]
pub use admin::{AdminStats, GuestGroupWithRsvp, RsvpStatus};
pub use age_category::AgeCategory;
pub use auth::{AuthResponse, AuthSession, AuthUser, LoginCredentials};
pub use dietary::DietaryPreferences;
pub use guest::{
    Guest, GuestGroup, GuestGroupInput, GuestGroupUpdate, GuestGroupWithCount, GuestInput,
    GuestUpdate,
};
pub use language::Language;
pub use location::Location;
