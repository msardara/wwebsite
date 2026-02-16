use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Sardinia,
    Tunisia,
    Nice,
}

#[allow(dead_code)]
impl Location {
    pub fn as_str(&self) -> &'static str {
        match self {
            Location::Sardinia => "sardinia",
            Location::Tunisia => "tunisia",
            Location::Nice => "nice",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sardinia" => Some(Location::Sardinia),
            "tunisia" => Some(Location::Tunisia),
            "nice" => Some(Location::Nice),
            _ => None,
        }
    }

    pub fn includes(&self, other: &Location) -> bool {
        self == other
    }

    // ========================================================================
    // DISPLAY METADATA
    //
    // Centralises all location-specific presentation details so that UI
    // components never need to hard-code flag emojis, image paths, colours,
    // or human-readable names.
    // ========================================================================

    /// Human-readable English name (e.g. `"Sardinia"`).
    pub fn display_name(&self) -> &'static str {
        match self {
            Location::Sardinia => "Sardinia",
            Location::Tunisia => "Tunisia",
            Location::Nice => "Nice",
        }
    }

    /// Flag emoji for inline text usage.
    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Location::Sardinia => "🇮🇹",
            Location::Tunisia => "🇹🇳",
            Location::Nice => "🇫🇷",
        }
    }

    /// Path to the flag image asset used in larger UI elements.
    pub fn flag_image(&self) -> &'static str {
        match self {
            Location::Sardinia => "/public/sardinia-flag.png",
            Location::Tunisia => "/public/tunisia-flag.png",
            Location::Nice => "/public/nice-flag.png",
        }
    }

    /// Tailwind CSS classes for a small inline badge:
    /// `(background, text, border)`.
    pub fn badge_css(&self) -> &'static str {
        match self {
            Location::Sardinia => "bg-blue-100 text-blue-800 border-blue-300",
            Location::Tunisia => "bg-red-100 text-red-800 border-red-300",
            Location::Nice => "bg-purple-100 text-purple-800 border-purple-300",
        }
    }

    /// Tailwind CSS classes for location guest-table headers:
    /// `(header_bg, header_border, title_text, count_text)`.
    pub fn table_header_colors(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Location::Sardinia => (
                "bg-blue-50",
                "border-blue-200",
                "text-blue-900",
                "text-blue-700",
            ),
            Location::Tunisia => (
                "bg-green-50",
                "border-green-200",
                "text-green-900",
                "text-green-700",
            ),
            Location::Nice => (
                "bg-purple-50",
                "border-purple-200",
                "text-purple-900",
                "text-purple-700",
            ),
        }
    }
}
