use crate::components::common::{
    AgeCategorySelector, DietaryCheckboxItem, LoadingButton, SuccessAlert, TranslatedErrorAlert,
};
use crate::contexts::{use_guest_context, use_supabase_rpc};
use crate::i18n::{use_translations, Translations};
use crate::styles::*;
use crate::types::{DietaryPreferences, Guest, GuestGroup, Location};
use leptos::*;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn RsvpPage() -> impl IntoView {
    let guest_context = use_guest_context();
    let translations = use_translations();

    view! {
        <div class="max-w-4xl mx-auto">
            <div class="text-center mb-6 animate-fade-in">
                <h1 class="text-5xl md:text-6xl font-serif font-light text-secondary-800 mb-6 tracking-wide">
                    {move || translations().t("rsvp.title")}
                </h1>
                <div class="w-24 h-0.5 bg-primary-400 mx-auto mb-6"></div>
                <p class="text-lg md:text-xl text-secondary-600 font-light mb-8">
                    {move || translations().t("rsvp.subtitle")}
                </p>
            </div>

            {move || {
                if let Some(guest) = guest_context.guest.get() {
                    view! {
                        <div>
                            <RsvpManager
                                guest=guest.clone()
                                translations=translations
                            />
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="text-center text-secondary-600 py-12">
                            <p class="text-xl font-light">{move || translations().t("rsvp.guest_not_found")}</p>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn RsvpManager(
    guest: GuestGroup,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let (guests, set_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (initial_load_complete, set_initial_load_complete) = create_signal(false);
    let (saving, set_saving) = create_signal(false);
    let (success, set_success) = create_signal(false);

    // Get available locations for this guest group from the locations array
    let available_locations_vec: Vec<Location> = guest
        .locations
        .iter()
        .filter_map(|loc_str| Location::from_str(loc_str))
        .filter(|loc| !loc.is_past())
        .collect();
    let (available_locations, _) = create_signal(available_locations_vec);

    // Track which guests are attending which locations
    // Map of guest_id -> Set of location strings
    let (guest_location_map, set_guest_location_map) =
        create_signal::<HashMap<String, HashSet<String>>>(HashMap::new());

    // Track additional notes for the guest group
    let (notes, set_notes) = create_signal(guest.additional_notes.clone().unwrap_or_default());

    // Load guests on mount and from localStorage
    let storage_key = store_value(format!("temp_guests_{}", guest.id));
    let location_storage_key = store_value(format!("guest_locations_{}", guest.id));

    let client = use_supabase_rpc();
    let guest_id_for_load = guest.id.clone();
    let invitation_code_for_load = guest.invitation_code.clone();

    spawn_local(async move {
        match client
            .get_guests(&guest_id_for_load, &invitation_code_for_load)
            .await
        {
            Ok(mut invitees_list) => {
                // Try to load temporary guests from localStorage
                if let Some(storage) = window().local_storage().ok().flatten() {
                    let key = storage_key.get_value();
                    if let Ok(Some(saved_temp_guests)) = storage.get_item(&key) {
                        if let Ok(temp_guests) =
                            serde_json::from_str::<Vec<Guest>>(&saved_temp_guests)
                        {
                            for temp_guest in temp_guests {
                                if temp_guest.id.starts_with("temp_") {
                                    invitees_list.push(temp_guest);
                                }
                            }
                        }
                    }
                }
                // Build location map from guest attending_locations arrays.
                // Use DB values directly — empty means the guest explicitly declined.
                let mut guest_loc_map: HashMap<String, HashSet<String>> = HashMap::new();
                for guest in &invitees_list {
                    let locs: HashSet<String> = guest.attending_locations.iter().cloned().collect();
                    guest_loc_map.insert(guest.id.clone(), locs);
                }

                set_guests.set(invitees_list.clone());

                // Try to load location selections from localStorage
                if let Some(storage) = window().local_storage().ok().flatten() {
                    let key = location_storage_key.get_value();
                    if let Ok(Some(saved_locations)) = storage.get_item(&key) {
                        if let Ok(saved_map) = serde_json::from_str::<
                            HashMap<String, HashSet<String>>,
                        >(&saved_locations)
                        {
                            // Merge saved locations with database locations
                            for (guest_id, locs) in saved_map {
                                guest_loc_map.entry(guest_id).or_default().extend(locs);
                            }
                        }
                    }
                }

                set_guest_location_map.set(guest_loc_map);
                set_loading.set(false);
                set_initial_load_complete.set(true);
            }
            Err(e) => {
                set_error.set(Some(format!("Error loading guests: {}", e)));
                set_loading.set(false);
                set_initial_load_complete.set(true);
            }
        }
    });

    // Save temporary guests to localStorage
    create_effect(move |_| {
        if !initial_load_complete.get() {
            return;
        }

        let guest_list = guests.get();
        let temp_guests: Vec<Guest> = guest_list
            .into_iter()
            .filter(|g| g.id.starts_with("temp_"))
            .collect();

        if let Some(storage) = window().local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(&temp_guests) {
                let key = storage_key.get_value();
                let _ = storage.set_item(&key, &json);
            }
        }
    });

    // Save guest location selections to localStorage
    create_effect(move |_| {
        if !initial_load_complete.get() {
            return;
        }

        let loc_map = guest_location_map.get();

        if let Some(storage) = window().local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(&loc_map) {
                let key = location_storage_key.get_value();
                let _ = storage.set_item(&key, &json);
            }
        }
    });

    let guest_id_for_closures = store_value(guest.id.clone());
    let guest_location_for_closures = store_value(guest.locations.clone());

    let add_guest = store_value(move |_| {
        let temp_id = format!("temp_{}", Uuid::new_v4());

        // Auto-select all locations for this guest group
        let selected_locs: Vec<String> = guest_location_for_closures.get_value().to_vec();

        let new_guest = Guest {
            id: temp_id.clone(),
            guest_group_id: guest_id_for_closures.get_value(),
            name: String::new(),
            attending_locations: selected_locs.clone(),
            dietary_preferences: DietaryPreferences::default(),
            age_category: crate::types::AgeCategory::default(),
            created_at: None,
            updated_at: None,
        };

        let mut current_guests = guests.get_untracked();
        current_guests.push(new_guest);
        set_guests.set(current_guests);

        // Update the location map
        let mut current_map = guest_location_map.get_untracked();
        let selected_set: HashSet<String> = selected_locs.into_iter().collect();
        current_map.insert(temp_id, selected_set);
        set_guest_location_map.set(current_map);
    });

    let update_guest = Callback::new(move |updated_guest: Guest| {
        let mut current_guests = guests.get_untracked();
        if let Some(index) = current_guests.iter().position(|g| g.id == updated_guest.id) {
            current_guests[index] = updated_guest;
            set_guests.set(current_guests);
        }
    });

    let delete_guest = Callback::new(move |guest_id: String| {
        let mut current_guests = guests.get_untracked();
        current_guests.retain(|g| g.id != guest_id);
        set_guests.set(current_guests);

        // Remove from location map
        let mut current_map = guest_location_map.get_untracked();
        current_map.remove(&guest_id);
        set_guest_location_map.set(current_map);
    });

    let toggle_guest_location = Callback::new(move |(guest_id, location): (String, String)| {
        let mut current_map = guest_location_map.get_untracked();
        let guest_locs = current_map.entry(guest_id).or_insert_with(HashSet::new);

        if guest_locs.contains(&location) {
            guest_locs.remove(&location);
        } else {
            guest_locs.insert(location);
        }

        set_guest_location_map.set(current_map);
    });

    let handle_submit = store_value(move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_saving.set(true);
        set_error.set(None);
        set_success.set(false);

        // Helper: scroll to top to make error/success banners visible
        let scroll_to_top = || {
            window().scroll_to_with_x_and_y(0.0, 0.0);
        };

        let all_guests = guests.get();
        let notes_value = notes.get();

        // Validate that all guests have names
        let empty_names: Vec<_> = all_guests
            .iter()
            .filter(|g| g.name.trim().is_empty())
            .collect();

        if !empty_names.is_empty() {
            set_saving.set(false);
            set_error.set(Some("rsvp.error_empty_names".to_string()));
            scroll_to_top();
            return;
        }

        // Read location map once, before validation and spawn_local
        let current_location_map = guest_location_map.get_untracked();

        let client = use_supabase_rpc();
        let group_id = guest_id_for_closures.get_value();
        let inv_code = guest.invitation_code.clone();

        spawn_local(async move {
            // Bulk save: creates/updates all guests + party_size + notes in one RPC call
            match client
                .save_rsvp(
                    &group_id,
                    &inv_code,
                    &all_guests,
                    &current_location_map,
                    &notes_value,
                )
                .await
            {
                Ok(_saved_guests) => {}
                Err(e) => {
                    set_saving.set(false);
                    set_error.set(Some(format!("Error saving RSVP: {}", e)));
                    scroll_to_top();
                    return;
                }
            }

            // Clear localStorage
            if let Some(storage) = window().local_storage().ok().flatten() {
                let key = storage_key.get_value();
                let _ = storage.remove_item(&key);
                let loc_key = location_storage_key.get_value();
                let _ = storage.remove_item(&loc_key);
            }

            set_saving.set(false);
            set_success.set(true);
            set_error.set(None);

            // Scroll to top so the success message is visible
            window().scroll_to_with_x_and_y(0.0, 0.0);

            // Reload the page after 5 seconds to show the saved state
            set_timeout(
                move || {
                    window().location().reload().ok();
                },
                std::time::Duration::from_secs(5),
            );
        });
    });

    view! {
        <div class="space-y-8">
            <Show
                when=move || success.get()
                fallback=move || view! {
                    <TranslatedErrorAlert message_key=Signal::derive(move || error.get()) />

                    <Show
                        when=move || !loading.get()
                        fallback=move || view! {
                            <div class="text-center text-secondary-600 py-16">
                                <div class="text-5xl mb-4">"⏳"</div>
                                <p class="text-lg font-light">{move || translations().t("common.loading")}</p>
                            </div>
                        }
                    >
                        <form on:submit=move |ev| handle_submit.with_value(|f| f(ev)) class="space-y-8">
                    // Guest List Section
                    <div class="bg-white rounded-2xl shadow-sm border border-primary-200 p-6 sm:p-8 lg:p-10">
                        <div class="flex items-center justify-between mb-8">
                            <div>
                                <h2 class="text-2xl md:text-3xl font-serif font-light text-secondary-800">
                                    {move || translations().t("rsvp.guest_list")}
                                    <span class="text-red-600">"*"</span>
                                </h2>
                                <p class="text-sm text-secondary-600 mt-2 font-light">
                                    {move || translations().t("rsvp.guest_list_help")}
                                </p>
                            </div>
                        </div>

                        <div class="space-y-4 mb-6">
                            <For
                                each=move || guests.get()
                                key=|g| g.id.clone()
                                children=move |guest: Guest| {
                                    view! {
                                        <GuestCard
                                            guest=guest.clone()
                                            available_locations=available_locations.get()
                                            guest_location_map=guest_location_map
                                            on_toggle_location=toggle_guest_location
                                            on_update=update_guest
                                            on_delete=delete_guest
                                            translations=translations
                                        />
                                    }
                                }
                            />
                        </div>

                        <button
                            type="button"
                            class="w-full py-4 px-6 bg-gradient-to-br from-primary-50 to-accent-50 hover:from-primary-100 hover:to-accent-100 text-secondary-700 font-light rounded-xl transition-all duration-200 border-2 border-dashed border-primary-300 hover:border-primary-400 hover:shadow-sm"
                            on:click=move |ev| add_guest.with_value(|f| f(ev))
                            disabled=move || saving.get()
                        >
                            {move || translations().t("rsvp.add_another_guest")}
                        </button>
                    </div>

                    // Additional notes section
                    <div class="bg-white rounded-2xl shadow-sm border border-primary-200 p-6 sm:p-8 lg:p-10">
                        <h2 class="text-2xl md:text-3xl font-serif font-light text-secondary-800 mb-4">
                            {move || translations().t("rsvp.notes")}
                        </h2>
                        <p class="text-sm text-secondary-600 mb-6 font-light">
                            {move || translations().t("rsvp.notes_help")}
                        </p>
                        <textarea
                            class="w-full px-4 sm:px-5 py-3 sm:py-4 border border-primary-200 rounded-xl focus:ring-2 focus:ring-primary-400 focus:border-transparent transition-all resize-none text-sm sm:text-base bg-primary-50/30 text-secondary-700 font-light"
                            rows="5"
                            placeholder=move || translations().t("rsvp.notes_placeholder")
                            prop:value=move || notes.get()
                            on:input=move |ev| set_notes.set(event_target_value(&ev))
                            disabled=move || saving.get()
                        />
                    </div>

                    <TranslatedErrorAlert message_key=Signal::derive(move || error.get()) />

                    <LoadingButton
                        loading=move || saving.get()
                        label=move || translations().t("rsvp.submit")
                        class=BUTTON_PRIMARY
                    />
                    </form>
                </Show>
            }
            >
                <SuccessAlert when=move || true>
                    <div class="text-5xl mb-4">"✓"</div>
                    <p class="text-2xl font-semibold mb-2">{move || translations().t("rsvp.success")}</p>
                    <p class="text-base mt-2 font-light">{move || translations().t("rsvp.success_refresh")}</p>
                </SuccessAlert>
            </Show>
        </div>
    }
}

#[component]
fn GuestCard(
    guest: Guest,
    available_locations: Vec<Location>,
    guest_location_map: ReadSignal<HashMap<String, HashSet<String>>>,
    #[prop(into)] on_toggle_location: Callback<(String, String)>,
    #[prop(into)] on_update: Callback<Guest>,
    #[prop(into)] on_delete: Callback<String>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let guest_id = guest.id.clone();
    let guest_id_for_locations = store_value(guest.id.clone());
    let (name, set_name) = create_signal(guest.name.clone());

    // Auto-focus the name input for new guests
    let input_ref = create_node_ref::<html::Input>();
    let is_new_guest = guest.name.is_empty() && guest.id.starts_with("temp_");

    if is_new_guest {
        request_animation_frame(move || {
            if let Some(input) = input_ref.get_untracked() {
                let _ = input.focus();
            }
        });
    }

    let (vegetarian, set_vegetarian) = create_signal(guest.dietary_preferences.vegetarian);
    let (vegan, set_vegan) = create_signal(guest.dietary_preferences.vegan);
    let (halal, set_halal) = create_signal(guest.dietary_preferences.halal);
    let (no_pork, set_no_pork) = create_signal(guest.dietary_preferences.no_pork);
    let (gluten_free, set_gluten_free) = create_signal(guest.dietary_preferences.gluten_free);
    let (other, set_other) = create_signal(guest.dietary_preferences.other.clone());
    let (age_category, set_age_category) = create_signal(guest.age_category.clone());

    let save_changes = store_value({
        let guest_id = guest_id.clone();
        move || {
            // Get attending locations from the map
            let attending_locs: Vec<String> = guest_location_map
                .get()
                .get(&guest_id)
                .map(|locs| locs.iter().cloned().collect())
                .unwrap_or_default();

            let updated_guest = Guest {
                id: guest_id.clone(),
                guest_group_id: guest.guest_group_id.clone(),
                name: name.get(),
                attending_locations: attending_locs,
                dietary_preferences: DietaryPreferences {
                    vegetarian: vegetarian.get(),
                    vegan: vegan.get(),
                    halal: halal.get(),
                    no_pork: no_pork.get(),
                    gluten_free: gluten_free.get(),
                    other: other.get(),
                },
                age_category: age_category.get(),
                created_at: None,
                updated_at: None,
            };

            // Only update local state — actual persistence happens on submit via save_rsvp
            on_update.call(updated_guest);
        }
    });

    let delete_guest = {
        let guest_id = guest_id.clone();
        move |_| {
            // Only update local state — actual deletion happens on submit via save_rsvp
            on_delete.call(guest_id.clone());
        }
    };

    let is_single_location = available_locations.len() == 1;
    let show_locations = !available_locations.is_empty();
    let available_locations_stored = store_value(available_locations);

    view! {
        <div class="bg-white border border-primary-200 p-4 sm:p-5 rounded-xl shadow-sm hover:shadow-md transition-all duration-200">
            <div class="space-y-4">
                <div class="flex items-start sm:items-center gap-2 w-full">
                    <input
                        node_ref=input_ref
                        type="text"
                        class="min-w-0 flex-1 px-4 py-2.5 border border-primary-200 rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent text-sm sm:text-base font-light text-secondary-800 bg-primary-50/30"
                        placeholder=move || translations().t("rsvp.invitee_name")
                        prop:value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        on:blur=move |_| save_changes.with_value(|f| f())
                    />
                    <button
                        type="button"
                        class="flex-shrink-0 w-10 h-10 flex items-center justify-center text-red-600 hover:bg-red-50 rounded-lg transition-colors border border-red-200"
                        on:click=delete_guest
                        title=move || translations().t("rsvp.delete_invitee")
                    >
                        "✕"
                    </button>
                </div>

                // Location selection
                <Show when=move || show_locations>
                    <div class="bg-gradient-to-br from-primary-50/50 to-accent-50/50 p-4 rounded-lg border border-primary-200">
                        {if is_single_location {
                            // Single location: plain yes/no, no location name shown
                            let loc_str = available_locations_stored.with_value(|v| {
                                v.first().map(|l| l.as_str().to_string()).unwrap_or_default()
                            });
                            let loc_str_for_click = loc_str.clone();
                            let guest_id_val = guest_id_for_locations.get_value();
                            let is_attending = move || {
                                guest_location_map
                                    .get()
                                    .get(&guest_id_val)
                                    .map(|locs| locs.contains(&loc_str))
                                    .unwrap_or(false)
                            };
                            view! {
                                <p class="text-xs font-medium text-secondary-700 mb-3">{move || translations().t("rsvp.attending_single_label")}</p>
                                <label class="flex items-center gap-2 cursor-pointer px-3 py-2 bg-white hover:bg-primary-50 rounded-lg border border-primary-200 transition-all duration-200 hover:shadow-sm w-fit">
                                    <input
                                        type="checkbox"
                                        class="w-4 h-4 text-secondary-600 rounded focus:ring-2 focus:ring-primary-400"
                                        prop:checked=is_attending
                                        on:change=move |_| {
                                            on_toggle_location.call((guest_id_for_locations.get_value(), loc_str_for_click.clone()))
                                        }
                                    />
                                    <span class="text-sm font-light text-secondary-700">
                                        {move || translations().t("rsvp.yes")}
                                    </span>
                                </label>
                            }.into_view()
                        } else {
                            // Multiple locations: show each location as a chip
                            view! {
                                <p class="text-xs font-medium text-secondary-700 mb-3">{move || translations().t("rsvp.attending_label")}</p>
                                <div class="flex flex-wrap gap-2">
                                    <For
                                        each=move || available_locations_stored.with_value(|v| v.clone())
                                        key=|loc| loc.as_str().to_string()
                                        children=move |location: Location| {
                                            let loc_str = location.as_str().to_string();
                                            let loc_str_for_click = loc_str.clone();
                                            let guest_id_val = guest_id_for_locations.get_value();

                                            let is_selected = move || {
                                                guest_location_map
                                                    .get()
                                                    .get(&guest_id_val)
                                                    .map(|locs| locs.contains(&loc_str))
                                                    .unwrap_or(false)
                                            };

                                            let i18n_key = format!("location.{}", location.as_str());
                                            let flag = location.flag_emoji();

                                            view! {
                                                <label class="flex items-center gap-2 cursor-pointer px-3 py-2 bg-white hover:bg-primary-50 rounded-lg border border-primary-200 transition-all duration-200 hover:shadow-sm">
                                                    <input
                                                        type="checkbox"
                                                        class="w-4 h-4 text-secondary-600 rounded focus:ring-2 focus:ring-primary-400"
                                                        prop:checked=is_selected
                                                        on:change=move |_| {
                                                            on_toggle_location.call((guest_id_for_locations.get_value(), loc_str_for_click.clone()))
                                                        }
                                                    />
                                                    <span class="text-sm font-light text-secondary-700">
                                                        {flag} " " {move || translations().t(&i18n_key)}
                                                    </span>
                                                </label>
                                            }
                                        }
                                    />
                                </div>
                            }.into_view()
                        }}
                    </div>
                </Show>

                // Age category selection
                <div class="bg-gradient-to-br from-primary-50/50 to-accent-50/50 p-4 rounded-lg border border-primary-200">
                    <p class="text-xs font-medium text-secondary-700 mb-3">{move || translations().t("rsvp.age_category")}</p>
                    <AgeCategorySelector
                        current=move || age_category.get()
                        on_change=move |cat| {
                            set_age_category.set(cat);
                            save_changes.with_value(|f| f());
                        }
                        radio_name=format!("age_category_{}", guest_id)
                        translations=translations
                    />
                </div>

                <div class="pl-1">
                    <p class="text-xs font-medium text-secondary-700 mb-3">{move || translations().t("rsvp.dietary_restrictions_label")}</p>
                    <div class="grid grid-cols-2 gap-2">
                        <DietaryCheckboxItem
                            checked=move || vegetarian.get()
                            on_change=move |v| { set_vegetarian.set(v); save_changes.with_value(|f| f()); }
                            label=move || translations().t("rsvp.vegetarian")
                        />
                        <DietaryCheckboxItem
                            checked=move || vegan.get()
                            on_change=move |v| { set_vegan.set(v); save_changes.with_value(|f| f()); }
                            label=move || translations().t("rsvp.vegan")
                        />
                        <DietaryCheckboxItem
                            checked=move || halal.get()
                            on_change=move |v| { set_halal.set(v); save_changes.with_value(|f| f()); }
                            label=move || translations().t("rsvp.halal")
                        />
                        <DietaryCheckboxItem
                            checked=move || no_pork.get()
                            on_change=move |v| { set_no_pork.set(v); save_changes.with_value(|f| f()); }
                            label=move || translations().t("rsvp.no_pork")
                        />
                        <DietaryCheckboxItem
                            checked=move || gluten_free.get()
                            on_change=move |v| { set_gluten_free.set(v); save_changes.with_value(|f| f()); }
                            label=move || translations().t("rsvp.gluten_free")
                        />
                    </div>

                    <div class="mt-4">
                        <input
                            type="text"
                            class="w-full h-8 px-2 py-2.5 text-xs text-secondary-700 font-light bg-transparent border-0 border-b border-primary-200 focus:border-primary-400 focus:outline-none placeholder-secondary-400"
                            placeholder=move || translations().t("rsvp.other_dietary")
                            prop:value=move || other.get()
                            on:input=move |ev| set_other.set(event_target_value(&ev))
                            on:blur=move |_| save_changes.with_value(|f| f())
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn LocationSection(
    location: String,
    title: String,
    flag: &'static str,
    location_attendance: ReadSignal<HashMap<String, bool>>,
    set_location_attendance: WriteSignal<HashMap<String, bool>>,
    location_notes: ReadSignal<HashMap<String, String>>,
    set_location_notes: WriteSignal<HashMap<String, String>>,
    saving: ReadSignal<bool>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let loc_key = store_value(location.clone());
    let loc_key_for_notes = store_value(location.clone());

    let is_attending = move || {
        location_attendance
            .get()
            .get(&loc_key.get_value())
            .copied()
            .unwrap_or(true)
    };

    let set_attending = move |attending: bool| {
        let mut map = location_attendance.get_untracked();
        map.insert(loc_key.get_value(), attending);
        set_location_attendance.set(map);
    };

    let notes_value = move || {
        location_notes
            .get()
            .get(&loc_key_for_notes.get_value())
            .cloned()
            .unwrap_or_default()
    };

    let set_notes = move |value: String| {
        let mut map = location_notes.get_untracked();
        map.insert(loc_key_for_notes.get_value(), value);
        set_location_notes.set(map);
    };

    view! {
        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8">
            <div class="flex items-center mb-6">
                <img src={flag} alt="Flag" class="w-14 h-10 sm:w-16 sm:h-12 mr-3 sm:mr-4 object-cover rounded shadow-md border border-gray-200"/>
                <h2 class="text-2xl sm:text-3xl font-serif font-bold text-gray-800">
                    {title}
                </h2>
            </div>



            <div class="space-y-6">
                <div>
                    <label class="block text-sm font-semibold text-gray-700 mb-3">
                        {move || translations().t("rsvp.attending")}
                    </label>
                    <div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
                        <button
                            type="button"
                            class=move || {
                                let base = "flex-1 py-3 px-4 rounded-lg font-semibold transition-all duration-200 ";
                                if is_attending() {
                                    format!("{}bg-green-500 text-white shadow-md", base)
                                } else {
                                    format!("{}bg-gray-100 text-gray-700 hover:bg-gray-200", base)
                                }
                            }
                            on:click=move |_| set_attending(true)
                            disabled=move || saving.get()
                        >
                            "✓ " {move || translations().t("rsvp.yes")}
                        </button>
                        <button
                            type="button"
                            class=move || {
                                let base = "flex-1 py-3 px-4 rounded-lg font-semibold transition-all duration-200 ";
                                if !is_attending() {
                                    format!("{}bg-red-500 text-white shadow-md", base)
                                } else {
                                    format!("{}bg-gray-100 text-gray-700 hover:bg-gray-200", base)
                                }
                            }
                            on:click=move |_| set_attending(false)
                            disabled=move || saving.get()
                        >
                            "✗ " {move || translations().t("rsvp.no")}
                        </button>
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                        {move || translations().t("rsvp.notes")}
                    </label>
                    <textarea
                        class="w-full px-3 sm:px-4 py-2 sm:py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent transition-all resize-none text-sm sm:text-base"
                        rows="4"
                        placeholder="Any special requests or messages?"
                        prop:value=notes_value
                        on:input=move |ev| set_notes(event_target_value(&ev))
                        disabled=move || saving.get()
                    />
                </div>
            </div>
        </div>
    }
}
