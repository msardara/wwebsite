//! Dietary preferences types and display helpers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DietaryPreferences {
    pub vegetarian: bool,
    pub vegan: bool,
    pub halal: bool,
    pub no_pork: bool,
    pub gluten_free: bool,
    pub other: String,
}

impl DietaryPreferences {
    /// Returns `true` if any dietary preference is set.
    pub fn has_any(&self) -> bool {
        self.vegetarian
            || self.vegan
            || self.halal
            || self.no_pork
            || self.gluten_free
            || !self.other.is_empty()
    }

    /// Returns a human-readable summary string with emoji icons.
    ///
    /// Example output: `"🥗 Vegetarian, 🌱 Vegan, 📝 No shellfish"`
    ///
    /// Returns `"-"` when no preferences are set.
    pub fn format_display(&self) -> String {
        let mut items = Vec::new();

        if self.vegetarian {
            items.push("🥗 Vegetarian".to_string());
        }
        if self.vegan {
            items.push("🌱 Vegan".to_string());
        }
        if self.halal {
            items.push("☪️ Halal".to_string());
        }
        if self.no_pork {
            items.push("🚫🐷 No Pork".to_string());
        }
        if self.gluten_free {
            items.push("🌾 Gluten-Free".to_string());
        }
        if !self.other.is_empty() {
            items.push(format!("📝 {}", self.other));
        }

        if items.is_empty() {
            "-".to_string()
        } else {
            items.join(", ")
        }
    }

    /// Returns a list of `(label, css_classes)` pairs for rendering as badges.
    ///
    /// Useful in admin views where each preference is shown as a styled badge.
    pub fn as_badges(&self) -> Vec<(&'static str, String)> {
        let mut badges = Vec::new();

        if self.vegetarian {
            badges.push((
                "🌱 Vegetarian",
                "bg-green-100 text-green-800 border-green-300".to_string(),
            ));
        }
        if self.vegan {
            badges.push((
                "🥬 Vegan",
                "bg-green-100 text-green-800 border-green-300".to_string(),
            ));
        }
        if self.halal {
            badges.push((
                "☪️ Halal",
                "bg-purple-100 text-purple-800 border-purple-300".to_string(),
            ));
        }
        if self.no_pork {
            badges.push((
                "🚫🐷 No Pork",
                "bg-pink-100 text-pink-800 border-pink-300".to_string(),
            ));
        }
        if self.gluten_free {
            badges.push((
                "🌾 Gluten-Free",
                "bg-yellow-100 text-yellow-800 border-yellow-300".to_string(),
            ));
        }

        badges
    }

    /// If the `other` field is non-empty, returns its badge info.
    pub fn other_badge(&self) -> Option<(String, &'static str)> {
        if self.other.is_empty() {
            None
        } else {
            Some((
                format!("ℹ️ {}", self.other),
                "bg-orange-100 text-orange-800 border-orange-300",
            ))
        }
    }
}
