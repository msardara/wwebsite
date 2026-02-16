use crate::i18n::use_translations;
use crate::types::{AgeCategory, Language, Location};
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

// ============================================================================
// REUSABLE UI COMPONENTS
// ============================================================================

/// A centered loading spinner with an optional message.
///
/// Replaces the duplicated spinner markup found in dashboard, rsvp, etc.
///
/// ```ignore
/// <LoadingSpinner message=move || translations().t("common.loading") />
/// ```
#[component]
pub fn LoadingSpinner(
    /// Optional message shown below the spinner. Pass `None` for spinner only.
    #[prop(optional, into)]
    message: Option<Signal<String>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 animate-fade-in">
            <div class="w-10 h-10 border-4 border-primary-200 border-t-primary-500 rounded-full animate-spin mb-4"></div>
            {message.map(|msg| view! {
                <p class="text-secondary-600 text-lg font-light">{msg}</p>
            })}
        </div>
    }
}

/// A styled error alert banner with a left border accent.
///
/// ```ignore
/// <ErrorAlert message=error_signal />
/// ```
#[component]
pub fn ErrorAlert(
    /// The error message to display. The component is only visible when this is `Some`.
    message: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <Show when=move || message.get().is_some()>
            <div class="bg-red-50 border-l-4 border-red-500 text-red-800 px-6 py-4 rounded-lg shadow-sm animate-fade-in">
                {move || message.get().unwrap_or_default()}
            </div>
        </Show>
    }
}

/// A styled error alert whose message is a translation key (looked up reactively).
///
/// ```ignore
/// <TranslatedErrorAlert message_key=error_key_signal />
/// ```
#[component]
pub fn TranslatedErrorAlert(
    /// The translation key (or raw fallback) to display. Visible when `Some`.
    message_key: Signal<Option<String>>,
) -> impl IntoView {
    let t = use_translations();
    view! {
        <Show when=move || message_key.get().is_some()>
            <div class="bg-red-50 border-l-4 border-red-500 text-red-800 px-6 py-4 rounded-lg shadow-sm animate-fade-in">
                {move || t().t(&message_key.get().unwrap_or_default())}
            </div>
        </Show>
    }
}

/// A styled success alert banner with a left border accent.
///
/// ```ignore
/// <SuccessAlert when=success_signal>
///     <p>"Saved!"</p>
/// </SuccessAlert>
/// ```
#[component]
pub fn SuccessAlert(
    /// Controls visibility.
    when: impl Fn() -> bool + 'static,
    children: Children,
) -> impl IntoView {
    // Render children eagerly (Children is FnOnce) and store the fragment,
    // then use a reactive closure to conditionally display it.
    let rendered = children();
    view! {
        <div style=move || if when() { "" } else { "display:none" }>
            <div class="bg-green-50 border-l-4 border-green-500 text-green-800 px-6 py-8 rounded-lg shadow-sm animate-fade-in text-center">
                {rendered.clone()}
            </div>
        </div>
    }
}

/// A submit / action button that shows an inline spinner while `loading` is true.
///
/// Replaces the duplicated `<Show when=loading fallback=...>` + spinner SVG
/// pattern found in invitation.rs, rsvp.rs, etc.
///
/// ```ignore
/// <LoadingButton loading=saving label=move || t().t("rsvp.submit") />
/// ```
#[component]
pub fn LoadingButton(
    /// When true the button shows a spinner and is disabled.
    loading: impl Fn() -> bool + 'static + Copy,
    /// The label shown when *not* loading.
    label: impl Fn() -> String + 'static + Copy,
    /// Optional loading text (defaults to the reactive `common.loading` translation).
    #[prop(optional, into)]
    loading_label: Option<Signal<String>>,
    /// HTML button type attribute (defaults to `"submit"`).
    #[prop(default = "submit")]
    button_type: &'static str,
    /// Extra CSS classes appended to the button.
    #[prop(default = crate::styles::BUTTON_PRIMARY)]
    class: &'static str,
) -> impl IntoView {
    let t = use_translations();
    let loading_text =
        loading_label.unwrap_or_else(|| Signal::derive(move || t().t("common.loading")));

    view! {
        <button
            type=button_type
            class=class
            disabled=loading
        >
            <Show
                when=loading
                fallback=move || view! { <span>{label()}</span> }
            >
                <span class="flex items-center justify-center">
                    <svg class="animate-spin h-5 w-5 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    {move || loading_text.get()}
                </span>
            </Show>
        </button>
    }
}

/// A simple icon + label + count card used in the admin dashboard.
///
/// Replaces the duplicated `LocationCard` and `DietaryCard` components.
///
/// ```ignore
/// <IconStatCard icon="🥗" label="Vegetarian" count=42 />
/// ```
#[component]
pub fn IconStatCard(icon: &'static str, label: &'static str, count: i32) -> impl IntoView {
    view! {
        <div class=crate::styles::INFO_CARD>
            <div class="flex items-center space-x-3">
                <span class="text-2xl">{icon}</span>
                <div>
                    <p class="text-sm font-medium text-gray-600">{label}</p>
                    <p class="text-2xl font-bold text-gray-900">{count}</p>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// LOCATION & DIETARY HELPERS
// ============================================================================

/// A small inline badge for a wedding location (flag + name, coloured).
///
/// Reads all presentation data from [`Location`] so there is a single source
/// of truth for flag emojis, display names, and Tailwind colours.
///
/// ```ignore
/// <LocationBadge location=Location::Sardinia />
/// ```
#[component]
pub fn LocationBadge(location: Location) -> impl IntoView {
    let css = location.badge_css();
    let label = format!("{} {}", location.flag_emoji(), location.display_name());
    view! {
        <span class={format!("px-2 py-1 text-xs font-semibold rounded-full border {}", css)}>
            {label}
        </span>
    }
}

/// A single dietary-preference checkbox used inside the RSVP guest card.
///
/// Extracts the repeated `<label><input type="checkbox" …/><span>…</span></label>`
/// pattern so the caller only provides the reactive state and a label.
///
/// ```ignore
/// <DietaryCheckboxItem
///     checked=move || vegetarian.get()
///     on_change=move |v| { set_vegetarian.set(v); save(); }
///     label=move || t().t("rsvp.vegetarian")
/// />
/// ```
#[component]
pub fn DietaryCheckboxItem(
    /// Whether the checkbox is currently checked.
    checked: impl Fn() -> bool + 'static + Copy,
    /// Called with the new checked state when the user toggles the checkbox.
    on_change: impl Fn(bool) + 'static + Copy,
    /// Reactive label text.
    label: impl Fn() -> String + 'static + Copy,
) -> impl IntoView {
    view! {
        <label class="flex items-center gap-2 cursor-pointer">
            <input
                type="checkbox"
                class="w-4 h-4 text-secondary-600 rounded focus:ring-2 focus:ring-primary-400"
                prop:checked=checked
                on:change=move |ev| on_change(event_target_checked(&ev))
            />
            <span class="text-xs text-secondary-700 font-light">{label}</span>
        </label>
    }
}

/// A group of radio buttons for selecting an [`AgeCategory`].
///
/// Replaces the three nearly-identical radio-button blocks found in
/// `rsvp.rs` `GuestCard` and `guests.rs` `GuestgroupModal`.
///
/// ```ignore
/// <AgeCategorySelector
///     current=move || age_category.get()
///     on_change=move |cat| { set_age_category.set(cat); save(); }
///     radio_name="age_cat_123"
///     translations=translations
/// />
/// ```
#[component]
pub fn AgeCategorySelector(
    /// Returns the currently selected age category.
    current: impl Fn() -> AgeCategory + 'static + Copy,
    /// Called when the user picks a different category.
    on_change: impl Fn(AgeCategory) + 'static + Copy,
    /// The HTML `name` attribute shared by all radio inputs in this group.
    #[prop(into)]
    radio_name: String,
    /// Translation function for labels.
    translations: impl Fn() -> crate::i18n::Translations + 'static + Copy,
) -> impl IntoView {
    let name1 = radio_name.clone();
    let name2 = radio_name.clone();
    let name3 = radio_name;

    view! {
        <div class="flex flex-wrap gap-2">
            <label class="flex items-center gap-2 cursor-pointer px-3 py-2 bg-white hover:bg-primary-50 rounded-lg border border-primary-200 transition-all duration-200 hover:shadow-sm">
                <input
                    type="radio"
                    name=name1
                    class="w-4 h-4 text-secondary-600"
                    prop:checked=move || current().as_str() == "adult"
                    on:change=move |_| on_change(AgeCategory::Adult)
                />
                <span class="text-sm font-light text-secondary-700">{move || translations().t("rsvp.adult")}</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer px-3 py-2 bg-white hover:bg-primary-50 rounded-lg border border-primary-200 transition-all duration-200 hover:shadow-sm">
                <input
                    type="radio"
                    name=name2
                    class="w-4 h-4 text-secondary-600"
                    prop:checked=move || current().as_str() == "child_under_3"
                    on:change=move |_| on_change(AgeCategory::ChildUnder3)
                />
                <span class="text-sm font-light text-secondary-700">{move || translations().t("rsvp.child_under_3")}</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer px-3 py-2 bg-white hover:bg-primary-50 rounded-lg border border-primary-200 transition-all duration-200 hover:shadow-sm">
                <input
                    type="radio"
                    name=name3
                    class="w-4 h-4 text-secondary-600"
                    prop:checked=move || current().as_str() == "child_under_10"
                    on:change=move |_| on_change(AgeCategory::ChildUnder10)
                />
                <span class="text-sm font-light text-secondary-700">{move || translations().t("rsvp.child_under_10")}</span>
            </label>
        </div>
    }
}

/// A colored stat card with an icon, title, numeric value, and accent color.
///
/// This is the single source of truth for overview-style stat cards, replacing
/// the duplicated `StatCard` in `dashboard.rs` and `SummaryCard` in `rsvps.rs`.
///
/// The `value` prop accepts either a static `i32` or a reactive closure via
/// `MaybeSignal`, so callers can pass a plain integer *or* a `move ||` closure.
///
/// ```ignore
/// // Static value (e.g. from a loaded struct field):
/// <StatCard icon="✉️" title="Total Invited" value=42 color="blue" />
///
/// // Reactive value (re-computed when signals change):
/// <StatCard icon="⏳" title="Pending" value=move || pending_count() color="yellow" />
/// ```
#[component]
pub fn StatCard(
    icon: &'static str,
    title: &'static str,
    #[prop(into)] value: MaybeSignal<i32>,
    color: &'static str,
) -> impl IntoView {
    let bg_color = crate::styles::stat_card_bg_color(color);
    let text_color = crate::styles::stat_card_text_color(color);

    view! {
        <div class={format!("rounded-lg shadow-md p-4 {}", bg_color)}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-xs font-medium text-gray-600 mb-1">{title}</p>
                    <p class={format!("text-2xl font-bold {}", text_color)}>{move || value.get()}</p>
                </div>
                <div class="text-3xl">{icon}</div>
            </div>
        </div>
    }
}
