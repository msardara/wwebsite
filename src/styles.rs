//! Common CSS class constants for consistent styling across the application.
//!
//! This module provides reusable CSS class strings that ensure consistent
//! styling throughout the wedding website. Use these constants instead of
//! hardcoding class strings in components.
//!
//! # Example
//! ```rust
//! use wedding_website::styles::*;
//! use leptos::*;
//!
//! let _view = view! {
//!     <button class=BUTTON_PRIMARY>"Click me"</button>
//!     <div class=CARD>"Card content"</div>
//! };
//! ```

// ============================================================================
// PAGE HEADERS
// ============================================================================

pub const PAGE_HEADER: &str = "page-header";
pub const SECTION_TITLE: &str = "section-title";
pub const SECTION_SUBTITLE: &str = "section-subtitle";

// ============================================================================
// CARDS
// ============================================================================

pub const CARD: &str = "card";
pub const CARD_HOVER: &str = "card-hover";
pub const CARD_INTERACTIVE: &str = "card-interactive";
pub const STAT_CARD: &str = "stat-card";
pub const INFO_CARD: &str = "info-card";

// ============================================================================
// BUTTONS (Trova RSVP style - full width with hover scale effect)
// ============================================================================

pub const BUTTON_BASE: &str = "btn";
pub const BUTTON_PRIMARY: &str = "btn-primary";
pub const BUTTON_SECONDARY: &str = "btn-secondary";
pub const BUTTON_ACCENT: &str = "btn-accent";
pub const BUTTON_DANGER: &str = "btn-danger";
pub const BUTTON_SUCCESS: &str = "btn-success";

// Inline buttons (not full width)
pub const BUTTON_PRIMARY_INLINE: &str = "btn-primary-inline";
pub const BUTTON_SECONDARY_INLINE: &str = "btn-secondary-inline";
pub const BUTTON_DANGER_INLINE: &str = "btn-danger-inline";

// Small buttons
pub const BUTTON_SMALL: &str = "btn-sm";

// Icon buttons
pub const BUTTON_ICON: &str = "btn-icon";

// ============================================================================
// FORM ELEMENTS
// ============================================================================

pub const FORM_LABEL: &str = "form-label";
pub const FORM_INPUT: &str = "form-input";
pub const FORM_SELECT: &str = "form-select";
pub const FORM_TEXTAREA: &str = "form-textarea";
pub const FORM_CHECKBOX: &str = "form-checkbox";
pub const FORM_ERROR: &str = "form-error";

// ============================================================================
// ALERTS
// ============================================================================

pub const ALERT_BASE: &str = "alert";
pub const ALERT_ERROR: &str = "alert-error";
pub const ALERT_SUCCESS: &str = "alert-success";
pub const ALERT_WARNING: &str = "alert-warning";
pub const ALERT_INFO: &str = "alert-info";

// ============================================================================
// BADGES
// ============================================================================

pub const BADGE_BASE: &str = "badge";
pub const BADGE_GREEN: &str = "badge-green";
pub const BADGE_YELLOW: &str = "badge-yellow";
pub const BADGE_BLUE: &str = "badge-blue";
pub const BADGE_RED: &str = "badge-red";
pub const BADGE_GRAY: &str = "badge-gray";

// ============================================================================
// DIETARY PREFERENCE BADGES
// ============================================================================

pub const DIETARY_BADGE_VEGETARIAN: &str = "dietary-badge-vegetarian";
pub const DIETARY_BADGE_VEGAN: &str = "dietary-badge-vegan";
pub const DIETARY_BADGE_GLUTEN_FREE: &str = "dietary-badge-gluten-free";
pub const DIETARY_BADGE_OTHER: &str = "dietary-badge-other";

// ============================================================================
// LOADING STATES
// ============================================================================

pub const LOADING_SPINNER: &str = "loading-spinner";
pub const LOADING_SPINNER_SM: &str = "loading-spinner-sm";
pub const LOADING_CONTAINER: &str = "loading-container";

// ============================================================================
// EMPTY STATES
// ============================================================================

pub const EMPTY_STATE: &str = "empty-state";
pub const EMPTY_STATE_ICON: &str = "empty-state-icon";
pub const EMPTY_STATE_TITLE: &str = "empty-state-title";
pub const EMPTY_STATE_MESSAGE: &str = "empty-state-message";

// ============================================================================
// TABLES
// ============================================================================

pub const TABLE_CONTAINER: &str = "table-container";
pub const TABLE: &str = "table";
pub const TABLE_HEADER: &str = "table-header";
pub const TABLE_HEADER_CELL: &str = "table-header-cell";
pub const TABLE_BODY: &str = "table-body";
pub const TABLE_CELL: &str = "table-cell";

// ============================================================================
// MODALS/DIALOGS
// ============================================================================

pub const MODAL_OVERLAY: &str = "modal-overlay";
pub const MODAL_CONTAINER: &str = "modal-container";
pub const MODAL_HEADER: &str = "modal-header";
pub const MODAL_TITLE: &str = "modal-title";
pub const MODAL_BODY: &str = "modal-body";
pub const MODAL_FOOTER: &str = "modal-footer";

// ============================================================================
// ANIMATIONS
// ============================================================================

pub const ANIMATE_FADE_IN: &str = "fade-in animate-fade-in";
pub const ANIMATE_SLIDE_IN: &str = "slide-in animate-slide-in";
pub const ANIMATE_PULSE_SUBTLE: &str = "pulse-subtle";

// ============================================================================
// COMMON LAYOUT PATTERNS
// ============================================================================

pub const SPACE_Y_6: &str = "space-y-6";
pub const FLEX_BETWEEN: &str = "flex items-center justify-between";
pub const FLEX_CENTER: &str = "flex items-center justify-center";
pub const GRID_2_COLS: &str = "grid grid-cols-1 md:grid-cols-2 gap-4";
pub const GRID_3_COLS: &str = "grid grid-cols-1 md:grid-cols-3 gap-4";
pub const GRID_4_COLS: &str = "grid grid-cols-1 md:grid-cols-4 gap-4";
pub const GRID_5_COLS: &str = "grid grid-cols-1 md:grid-cols-5 gap-4";

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Combines multiple CSS classes into a single string.
///
/// # Example
/// ```rust
/// use wedding_website::styles::{combine_classes, CARD};
/// let classes = combine_classes(&[CARD, "p-4", "mb-2"]);
/// // Returns: "card p-4 mb-2"
/// ```
pub fn combine_classes(classes: &[&str]) -> String {
    classes.join(" ")
}

/// Returns a stat card color class based on the color name.
///
/// # Example
/// ```rust
/// use wedding_website::styles::{stat_card_bg_color, stat_card_text_color};
/// let bg = stat_card_bg_color("blue");
/// let text = stat_card_text_color("blue");
/// ```
pub fn stat_card_bg_color(color: &str) -> &'static str {
    match color {
        "green" => "bg-green-50",
        "blue" => "bg-blue-50",
        "yellow" => "bg-yellow-50",
        "purple" => "bg-purple-50",
        "indigo" => "bg-indigo-50",
        "pink" => "bg-pink-50",
        "amber" => "bg-amber-50",
        "orange" => "bg-orange-50",
        "red" => "bg-red-50",
        _ => "bg-gray-50",
    }
}

/// Returns a stat card text color class based on the color name.
pub fn stat_card_text_color(color: &str) -> &'static str {
    match color {
        "green" => "text-green-600",
        "blue" => "text-blue-600",
        "yellow" => "text-yellow-600",
        "purple" => "text-purple-600",
        "indigo" => "text-indigo-600",
        "pink" => "text-pink-600",
        "amber" => "text-amber-600",
        "orange" => "text-orange-600",
        "red" => "text-red-600",
        _ => "text-gray-600",
    }
}

/// Returns a badge class based on the badge type.
pub fn badge_for_type(badge_type: &str) -> &'static str {
    match badge_type {
        "success" | "green" => BADGE_GREEN,
        "warning" | "yellow" => BADGE_YELLOW,
        "info" | "blue" => BADGE_BLUE,
        "error" | "danger" | "red" => BADGE_RED,
        _ => BADGE_GRAY,
    }
}

/// Returns a dietary badge class based on the dietary type.
pub fn dietary_badge(dietary_type: &str) -> &'static str {
    match dietary_type {
        "vegetarian" => DIETARY_BADGE_VEGETARIAN,
        "vegan" => DIETARY_BADGE_VEGAN,
        "gluten_free" | "gluten-free" => DIETARY_BADGE_GLUTEN_FREE,
        "other" => DIETARY_BADGE_OTHER,
        _ => BADGE_GRAY,
    }
}

// ============================================================================
// COMMON COMPONENT STRUCTURES
// ============================================================================

/// Common page layout structure with consistent spacing.
pub const PAGE_LAYOUT: &str = "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8";

/// Common container for admin pages.
pub const ADMIN_CONTAINER: &str = "space-y-6";

/// Common page header structure.
pub const PAGE_HEADER_CONTAINER: &str = "flex flex-wrap items-center justify-between gap-2 mb-4";

/// Common refresh button (used in admin pages) - uses inline style.
pub const REFRESH_BUTTON: &str = "btn-primary-inline";

/// Common search/filter section.
pub const FILTER_SECTION: &str = "bg-white rounded-lg shadow-md p-3 sm:p-4";

/// Common grid for filter inputs.
pub const FILTER_GRID: &str = "grid grid-cols-1 md:grid-cols-3 gap-4";
