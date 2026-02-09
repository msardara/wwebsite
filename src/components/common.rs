use crate::constants::LANGUAGE_KEY;
use crate::types::Language;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

/// Shared language selector component
/// Displays language flags/buttons and handles language switching with persistence
#[component]
pub fn LanguageSelector(
    language: ReadSignal<Language>,
    on_change: impl Fn(Language) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="flex items-center space-x-2" role="group" aria-label="Language selector">
            <LanguageButton
                language=language
                target=Language::English
                on_change=on_change
                flag="🇬🇧"
                label="EN"
                aria_label="Switch to English"
            />
            <LanguageButton
                language=language
                target=Language::French
                on_change=on_change
                flag="🇫🇷"
                label="FR"
                aria_label="Switch to French"
            />
            <LanguageButton
                language=language
                target=Language::Italian
                on_change=on_change
                flag="🇮🇹"
                label="IT"
                aria_label="Switch to Italian"
            />
        </div>
    }
}

/// Individual language button component
#[component]
fn LanguageButton(
    language: ReadSignal<Language>,
    target: Language,
    on_change: impl Fn(Language) + 'static + Copy,
    flag: &'static str,
    label: &'static str,
    aria_label: &'static str,
) -> impl IntoView {
    let is_active = move || language.get() == target;

    view! {
        <button
            class=move || {
                let base = "px-3 py-1 rounded-md text-sm transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-primary-500 ";
                if is_active() {
                    format!("{}bg-primary-400 text-white scale-110 font-semibold", base)
                } else {
                    format!("{}bg-gray-100 hover:bg-gray-200 text-gray-700", base)
                }
            }
            on:click=move |_| on_change(target)
            aria-label=aria_label
            aria-pressed=move || if is_active() { "true" } else { "false" }
            type="button"
        >
            <span class="flex items-center space-x-1">
                <span>{flag}</span>
                <span class="hidden sm:inline">{label}</span>
            </span>
        </button>
    }
}

/// Helper function to change language and persist to localStorage
#[allow(dead_code)]
pub fn change_language_with_persistence(set_language: WriteSignal<Language>, lang: Language) {
    set_language.set(lang);
    let _ = LocalStorage::set(LANGUAGE_KEY, lang.code());
}

/// Helper function to load language from localStorage
#[allow(dead_code)]
pub fn load_language_preference() -> Option<Language> {
    LocalStorage::get::<String>(LANGUAGE_KEY)
        .ok()
        .map(|code| Language::from_code(&code))
}
