use crate::constants::{AUTHENTICATED_GUEST_KEY, LANGUAGE_KEY};
use crate::types::{GuestGroup, Language};
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

/// Hook to get the GuestContext from Leptos context.
///
/// Replaces the boilerplate repeated in every component:
/// ```ignore
/// let guest_context = use_context::<GuestContext>().expect("GuestContext not found");
/// ```
pub fn use_guest_context() -> GuestContext {
    use_context::<GuestContext>()
        .expect("GuestContext not found. Make sure it's provided at the app level.")
}

/// Hook to get the language signal and a change handler that persists to localStorage.
///
/// Replaces the boilerplate repeated in layout.rs, invitation.rs, etc.:
/// ```ignore
/// let language = use_context::<ReadSignal<Language>>().expect("...");
/// let set_language = use_context::<WriteSignal<Language>>().expect("...");
/// let change_language = move |lang: Language| { set_language.set(lang); LocalStorage::set(...); };
/// ```
///
/// Returns `(language_signal, change_language_fn)`.
pub fn use_language() -> (ReadSignal<Language>, impl Fn(Language) + Copy) {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let set_language =
        use_context::<WriteSignal<Language>>().expect("Language setter context not found");
    let change = move |lang: Language| {
        set_language.set(lang);
        let _ = LocalStorage::set(LANGUAGE_KEY, lang.code());
    };
    (language, change)
}

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

    /// Get the current guest's locations
    #[allow(dead_code)]
    pub fn get_locations(&self) -> Option<Vec<String>> {
        self.guest.get().map(|g| g.locations)
    }

    /// Check if the guest can see content for a specific location
    pub fn can_see_location(&self, location: &str) -> bool {
        match self.guest.get() {
            Some(guest) => guest
                .locations
                .iter()
                .any(|loc| loc.eq_ignore_ascii_case(location)),
            None => false, // No guest authenticated, can't see any location
        }
    }
}
