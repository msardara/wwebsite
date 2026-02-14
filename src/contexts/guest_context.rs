use crate::constants::{AUTHENTICATED_GUEST_KEY, LANGUAGE_KEY};
use crate::types::{GuestGroup, Language, Location};
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

/// Context for managing the authenticated guest
#[derive(Clone, Copy, Debug)]
pub struct GuestContext {
    pub guest: ReadSignal<Option<GuestGroup>>,
    pub set_guest: WriteSignal<Option<GuestGroup>>,
    pub set_language: WriteSignal<Language>,
}

impl GuestContext {
    /// Create a new guest context
    pub fn new(set_language: WriteSignal<Language>) -> Self {
        let (guest, set_guest) = create_signal::<Option<GuestGroup>>(None);

        // Try to load guest from local storage on initialization
        if let Ok(stored_guest) = LocalStorage::get::<GuestGroup>(AUTHENTICATED_GUEST_KEY) {
            // Also restore the guest's default language
            let lang = Language::from_code(&stored_guest.default_language);
            set_language.set(lang);
            let _ = LocalStorage::set(LANGUAGE_KEY, lang.code());

            set_guest.set(Some(stored_guest));
        }

        Self {
            guest,
            set_guest,
            set_language,
        }
    }

    /// Set the authenticated guest and save to local storage
    /// Also sets the guest's default language
    pub fn login(&self, guest: GuestGroup) {
        // Set the guest's default language
        let lang = Language::from_code(&guest.default_language);
        self.set_language.set(lang);
        let _ = LocalStorage::set(LANGUAGE_KEY, lang.code());

        // Save guest to local storage
        let _ = LocalStorage::set(AUTHENTICATED_GUEST_KEY, guest.clone());
        self.set_guest.set(Some(guest));
    }

    /// Clear the authenticated guest
    pub fn logout(&self) {
        LocalStorage::delete(AUTHENTICATED_GUEST_KEY);
        self.set_guest.set(None);
    }

    /// Check if a guest is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.guest.get().is_some()
    }

    /// Get the current guest's location
    #[allow(dead_code)]
    pub fn get_location(&self) -> Option<Location> {
        self.guest.get().map(|g| g.location)
    }

    /// Check if the guest can see content for a specific location
    pub fn can_see_location(&self, location: &str) -> bool {
        match self.guest.get() {
            Some(guest) => match guest.location {
                Location::Both => true,
                Location::Sardinia => location.eq_ignore_ascii_case("sardinia"),
                Location::Tunisia => location.eq_ignore_ascii_case("tunisia"),
            },
            None => false, // No guest authenticated, can't see any location
        }
    }
}
