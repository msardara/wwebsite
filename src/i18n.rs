//! Internationalisation (i18n) module.
//!
//! Translation strings are split into per-language submodules for easier
//! maintenance:
//!
//! - [`en`] — English
//! - [`fr`] — French
//! - [`it`] — Italian

mod en;
mod fr;
mod it;

use crate::types::Language;
use leptos::*;
use std::collections::HashMap;

pub struct Translations {
    language: Language,
}

/// Hook that returns a reactive translations accessor.
///
/// Replaces the boilerplate that was repeated in every page/component:
/// ```ignore
/// let language = use_context::<ReadSignal<Language>>().expect("...");
/// let translations = move || Translations::new(language.get());
/// ```
///
/// Usage:
/// ```ignore
/// let t = use_translations();
/// view! { <p>{move || t().t("key")}</p> }
/// ```
pub fn use_translations() -> impl Fn() -> Translations + Copy {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    move || Translations::new(language.get())
}

impl Translations {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn t(&self, key: &str) -> String {
        let translations = self.get_translations();

        translations
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    fn get_translations(&self) -> HashMap<&'static str, &'static str> {
        match self.language {
            Language::English => en::translations(),
            Language::French => fr::translations(),
            Language::Italian => it::translations(),
        }
    }
}
