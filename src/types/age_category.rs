use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum AgeCategory {
    #[default]
    #[serde(rename = "adult")]
    Adult,
    #[serde(rename = "child_under_3")]
    ChildUnder3,
    #[serde(rename = "child_under_10")]
    ChildUnder10,
}

impl AgeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgeCategory::Adult => "adult",
            AgeCategory::ChildUnder3 => "child_under_3",
            AgeCategory::ChildUnder10 => "child_under_10",
        }
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "adult" => Some(AgeCategory::Adult),
            "child_under_3" => Some(AgeCategory::ChildUnder3),
            "child_under_10" => Some(AgeCategory::ChildUnder10),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AgeCategory::Adult => "Adult",
            AgeCategory::ChildUnder3 => "Child (< 3 years)",
            AgeCategory::ChildUnder10 => "Child (< 10 years)",
        }
    }
}
