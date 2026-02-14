// Application constants
// This module contains all constant values used throughout the application

#![allow(dead_code)]

// ============================================================================
// Storage Keys
// ============================================================================

/// Key for storing admin session in localStorage
pub const ADMIN_SESSION_KEY: &str = "admin_session";

/// Key for storing authenticated guest in localStorage
pub const AUTHENTICATED_GUEST_KEY: &str = "authenticated_guest";

/// Key for storing language preference in localStorage
pub const LANGUAGE_KEY: &str = "language";

// ============================================================================
// API Configuration
// ============================================================================

/// Supabase REST API path
pub const SUPABASE_REST_PATH: &str = "/rest/v1";

/// Supabase Auth API path
pub const SUPABASE_AUTH_PATH: &str = "/auth/v1";

/// Supabase Storage API path
pub const SUPABASE_STORAGE_PATH: &str = "/storage/v1";

// ============================================================================
// Storage Buckets
// ============================================================================

/// Wedding photos storage bucket name
pub const WEDDING_PHOTOS_BUCKET: &str = "wedding-photos";

// ============================================================================
// Database Tables
// ============================================================================

/// Guest groups table name
pub const TABLE_GUEST_GROUPS: &str = "guest_groups";

/// Guests table name
pub const TABLE_GUESTS: &str = "guests";

/// Photos table name
pub const TABLE_PHOTOS: &str = "photos";

/// Content table name
pub const TABLE_CONTENT: &str = "content";

// ============================================================================
// UI Constants
// ============================================================================

/// Default animation duration in milliseconds
pub const ANIMATION_DURATION_MS: u32 = 200;

/// Toast notification display duration in milliseconds
pub const TOAST_DURATION_MS: u32 = 3000;

/// Maximum file size for photo uploads (10MB)
pub const MAX_PHOTO_SIZE_BYTES: usize = 10 * 1024 * 1024;

/// Allowed photo file extensions
pub const ALLOWED_PHOTO_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];

// ============================================================================
// Validation Constants
// ============================================================================

/// Minimum invitation code length
pub const MIN_CODE_LENGTH: usize = 4;

/// Maximum invitation code length
pub const MAX_CODE_LENGTH: usize = 12;

/// Maximum guest name length
pub const MAX_NAME_LENGTH: usize = 100;

/// Maximum email length
pub const MAX_EMAIL_LENGTH: usize = 254;

/// Maximum party size
pub const MAX_PARTY_SIZE: i32 = 20;

/// Maximum dietary notes length
pub const MAX_DIETARY_NOTES_LENGTH: usize = 500;

/// Maximum additional notes length
pub const MAX_ADDITIONAL_NOTES_LENGTH: usize = 1000;

// ============================================================================
// Location Values
// ============================================================================

/// Sardinia location identifier
pub const LOCATION_SARDINIA: &str = "sardinia";

/// Tunisia location identifier
pub const LOCATION_TUNISIA: &str = "tunisia";

// ============================================================================
// Error Messages (English defaults)
// ============================================================================

/// Generic error message
pub const ERROR_GENERIC: &str = "An unexpected error occurred. Please try again.";

/// Network error message
pub const ERROR_NETWORK: &str = "Network error. Please check your connection.";

/// Authentication error message
pub const ERROR_AUTH: &str = "Authentication failed. Please check your credentials.";

/// Not found error message
pub const ERROR_NOT_FOUND: &str = "The requested resource was not found.";

/// Validation error message
pub const ERROR_VALIDATION: &str = "Please check your input and try again.";

// ============================================================================
// Success Messages (English defaults)
// ============================================================================

/// Generic success message
pub const SUCCESS_SAVED: &str = "Changes saved successfully!";

/// RSVP submitted message
pub const SUCCESS_RSVP_SUBMITTED: &str = "Your RSVP has been submitted!";

/// Photo uploaded message
pub const SUCCESS_PHOTO_UPLOADED: &str = "Photo uploaded successfully!";

// ============================================================================
// Retry Configuration
// ============================================================================

/// Maximum number of retry attempts for failed requests
pub const MAX_RETRY_ATTEMPTS: u32 = 3;

/// Initial retry delay in milliseconds
pub const RETRY_DELAY_MS: u64 = 1000;

/// Retry backoff multiplier
pub const RETRY_BACKOFF_MULTIPLIER: u64 = 2;

// ============================================================================
// Pagination
// ============================================================================

/// Default page size for list views
pub const DEFAULT_PAGE_SIZE: usize = 50;

/// Maximum page size
pub const MAX_PAGE_SIZE: usize = 100;

// ============================================================================
// Session Configuration
// ============================================================================

/// Session timeout in seconds (24 hours)
pub const SESSION_TIMEOUT_SECONDS: i64 = 86400;

/// Token refresh threshold in seconds (1 hour before expiry)
pub const TOKEN_REFRESH_THRESHOLD_SECONDS: i64 = 3600;

// ============================================================================
// Feature Flags
// ============================================================================

/// Enable debug logging (controlled by debug_assertions)
#[cfg(debug_assertions)]
pub const DEBUG_LOGGING: bool = true;

#[cfg(not(debug_assertions))]
pub const DEBUG_LOGGING: bool = false;

/// Enable performance monitoring
pub const ENABLE_PERFORMANCE_MONITORING: bool = false;

// ============================================================================
// Charset for Code Generation
// ============================================================================

/// Character set for generating invitation codes (alphanumeric, no ambiguous chars)
pub const CODE_CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
