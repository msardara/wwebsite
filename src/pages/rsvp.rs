use crate::contexts::{use_supabase_rpc, GuestContext};
use crate::i18n::Translations;
use crate::styles::*;
use crate::types::{DietaryPreferences, Guest, GuestGroup, Language, Location, Rsvp};
use leptos::*;
use std::collections::HashSet;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn RsvpPage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");
    let translations = move || Translations::new(language.get());

    let (existing_rsvps, set_existing_rsvps) = create_signal::<Vec<Rsvp>>(Vec::new());
    let (loading, set_loading) = create_signal(true);

    // Load existing RSVPs for the authenticated guest
    create_effect(move |_| {
        if let Some(guest) = guest_context.guest.get() {
            set_loading.set(true);
            let client = use_supabase_rpc();
            let guest_id = guest.id.clone();

            spawn_local(async move {
                match client.get_rsvps_by_guest(&guest_id).await {
                    Ok(rsvps) => {
                        set_existing_rsvps.set(rsvps);
                        set_loading.set(false);
                    }
                    Err(_) => {
                        set_existing_rsvps.set(Vec::new());
                        set_loading.set(false);
                    }
                }
            });
        }
    });

    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6">
            <div class="text-center mb-8 sm:mb-12 animate-fade-in">
                <h1 class="text-3xl sm:text-4xl md:text-5xl font-serif font-bold text-primary-600 mb-3 sm:mb-4">
                    {move || translations().t("rsvp.title")}
                </h1>
                <p class="text-base sm:text-lg text-gray-600">
                    {move || translations().t("rsvp.subtitle")}
                </p>
            </div>

            <Show
                when=move || !loading.get()
                fallback=move || view! {
                    <div class="flex items-center justify-center py-12">
                        <span class="text-gray-600">{move || translations().t("common.loading")}</span>
                    </div>
                }
            >
                {move || {
                    if let Some(guest) = guest_context.guest.get() {
                        let rsvps = existing_rsvps.get();

                        view! {
                            <div>
                                <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 mb-6 sm:mb-8 animate-fade-in">
                                    <div class="text-center">
                                        <h2 class="text-2xl sm:text-3xl font-serif text-gray-800 mb-2">
                                            {move || translations().t("rsvp.welcome")} ", "
                                            <span class="text-primary-600">{guest.name.clone()}</span> "!"
                                        </h2>
                                        {match guest.location {
                                            Location::Both => view! {
                                                <p class="text-sm sm:text-base text-gray-600">
                                                    {move || translations().t("rsvp.both_events")}
                                                </p>
                                            }.into_view(),
                                            _ => view! {}.into_view(),
                                        }}
                                    </div>
                                </div>

                                <RsvpManager
                                    guest=guest.clone()
                                    existing_rsvps=rsvps
                                    translations=translations
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="text-center text-gray-600">
                                "Guest not found"
                            </div>
                        }.into_view()
                    }
                }}
            </Show>
        </div>
    }
}

#[component]
fn RsvpManager(
    guest: GuestGroup,
    existing_rsvps: Vec<Rsvp>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    // Master guest list with dietary preferences
    let (guests, set_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (initial_load_complete, set_initial_load_complete) = create_signal(false);

    // Load guests on mount and from localStorage
    let storage_key = store_value(format!("temp_guests_{}", guest.id));
    
    // Load guests only once on mount
    let client = use_supabase_rpc();
    let guest_id_for_load = guest.id.clone();
    let invitation_code_for_load = guest.invitation_code.clone();

    spawn_local(async move {
        match client.get_guests(&guest_id_for_load, &invitation_code_for_load).await {
            Ok(mut invitees_list) => {
                // Try to load temporary guests from localStorage
                if let Some(storage) = window().local_storage().ok().flatten() {
                    let key = storage_key.get_value();
                    web_sys::console::log_1(&format!("🔍 Loading from localStorage key: {}", key).into());
                    if let Ok(Some(saved_temp_guests)) = storage.get_item(&key) {
                        web_sys::console::log_1(&format!("📦 Found saved data: {}", saved_temp_guests).into());
                        if let Ok(temp_guests) = serde_json::from_str::<Vec<Guest>>(&saved_temp_guests) {
                            web_sys::console::log_1(&format!("✅ Loaded {} temp guests", temp_guests.len()).into());
                            // Add temp guests that aren't already in the DB list
                            for temp_guest in temp_guests {
                                if temp_guest.id.starts_with("temp_") {
                                    invitees_list.push(temp_guest);
                                }
                            }
                        } else {
                            web_sys::console::log_1(&"❌ Failed to parse temp guests JSON".into());
                        }
                    } else {
                        web_sys::console::log_1(&"📭 No temp guests found in localStorage".into());
                    }
                }
                web_sys::console::log_1(&format!("📋 Total guests loaded: {}", invitees_list.len()).into());
                set_guests.set(invitees_list);
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

    // Save temporary guests to localStorage whenever the guest list changes
    // But only after initial load is complete to avoid overwriting during load
    create_effect(move |_| {
        // Don't save until initial load is complete
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
                web_sys::console::log_1(&format!("💾 Saving {} temp guests to localStorage key: {}", temp_guests.len(), key).into());
                web_sys::console::log_1(&format!("📝 Data: {}", json).into());
                match storage.set_item(&key, &json) {
                    Ok(_) => web_sys::console::log_1(&"✅ Saved successfully".into()),
                    Err(e) => web_sys::console::log_1(&format!("❌ Failed to save: {:?}", e).into()),
                }
            }
        }
    });

    view! {
        <div class="space-y-8">
            <Show when=move || error.get().is_some()>
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
                    {move || error.get().unwrap_or_default()}
                </div>
            </Show>

            <Show
                when=move || !loading.get()
                fallback=move || view! {
                    <div class="text-center text-gray-600 py-12">
                        {move || translations().t("common.loading")}
                    </div>
                }
            >
                <GuestListManager
                    guest_group=guest.clone()
                    guests=guests
                    set_guests=set_guests
                    translations=translations
                />

                {match guest.location {
                    Location::Both => view! {
                        <div class="space-y-8">
                            <LocationAttendanceSection
                                guest_group=guest.clone()
                                location=Location::Sardinia
                                location_title=move || translations().t("events.sardinia")
                                flag="/public/sardinia-flag.png"
                                guests=guests
                                existing_rsvp=existing_rsvps.iter().find(|r| r.location == "sardinia").cloned()
                                translations=translations
                            />
                            <LocationAttendanceSection
                                guest_group=guest.clone()
                                location=Location::Tunisia
                                location_title=move || translations().t("events.tunisia")
                                flag="/public/tunisia-flag.png"
                                guests=guests
                                existing_rsvp=existing_rsvps.iter().find(|r| r.location == "tunisia").cloned()
                                translations=translations
                            />
                        </div>
                    }.into_view(),
                    Location::Sardinia => view! {
                        <LocationAttendanceSection
                            guest_group=guest.clone()
                            location=Location::Sardinia
                            location_title=move || translations().t("events.sardinia")
                            flag="/public/sardinia-flag.png"
                            guests=guests
                            existing_rsvp=existing_rsvps.iter().find(|r| r.location == "sardinia").cloned()
                            translations=translations
                        />
                    }.into_view(),
                    Location::Tunisia => view! {
                        <LocationAttendanceSection
                            guest_group=guest.clone()
                            location=Location::Tunisia
                            location_title=move || translations().t("events.tunisia")
                            flag="/public/tunisia-flag.png"
                            guests=guests
                            existing_rsvp=existing_rsvps.iter().find(|r| r.location == "tunisia").cloned()
                            translations=translations
                        />
                    }.into_view(),
                }}
            </Show>
        </div>
    }
}

#[component]
fn GuestListManager(
    guest_group: GuestGroup,
    guests: ReadSignal<Vec<Guest>>,
    set_guests: WriteSignal<Vec<Guest>>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let guest_group_id = store_value(guest_group.id.clone());
    let invitation_code = store_value(guest_group.invitation_code.clone());

    let add_guest = move |_| {
        let guest_group_id = guest_group_id.get_value();
        let temp_id = format!("temp_{}", Uuid::new_v4());
        let new_guest = Guest {
            id: temp_id,
            guest_group_id,
            name: String::new(),
            dietary_preferences: DietaryPreferences::default(),
            created_at: None,
            updated_at: None,
        };

        let mut current = guests.get_untracked();
        current.push(new_guest);
        set_guests.set(current);
    };

    view! {
        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 animate-fade-in">
            <h2 class="text-2xl sm:text-3xl font-serif font-bold text-gray-800 mb-2">
                "Guest List & Dietary Preferences"
            </h2>
            <p class="text-sm text-gray-600 mb-6">
                "Manage your guest list and dietary restrictions here. You can then select which guests attend each location below."
            </p>

            <div class="space-y-3 mb-4">
                <For
                    each=move || guests.get()
                    key=|guest| guest.id.clone()
                    children=move |guest: Guest| {
                        let on_update = Callback::new(move |updated: Guest| {
                            let mut current = guests.get_untracked();
                            if let Some(pos) = current.iter().position(|g| g.id == updated.id) {
                                current[pos] = updated;
                            }
                            set_guests.set(current);
                        });
                        let on_delete = Callback::new(move |id: String| {
                            let mut current = guests.get_untracked();
                            current.retain(|g| g.id != id);
                            set_guests.set(current);
                        });
                        view! {
                            <GuestCard
                                guest=guest.clone()
                                guest_group_id=guest_group_id.get_value()
                                invitation_code=invitation_code.get_value()
                                on_update=on_update
                                on_delete=on_delete
                                translations=translations
                            />
                        }
                    }
                />
            </div>

            <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 pt-4 border-t border-gray-200">
                <span class="text-sm text-gray-600">
                    {move || guests.get().len()} " " {move || if guests.get().len() == 1 { "guest" } else { "guests" }}
                </span>
                <button
                    class="w-full sm:w-auto px-4 py-2 bg-secondary-500 text-gray-900 rounded-lg hover:bg-secondary-600 transition-all font-semibold shadow-md border-2 border-secondary-700"
                    on:click=add_guest
                >
                    "+ " {move || translations().t("rsvp.add_invitee")}
                </button>
            </div>
        </div>
    }
}

#[component]
fn GuestCard(
    guest: Guest,
    guest_group_id: String,
    invitation_code: String,
    #[prop(into)] on_update: Callback<Guest>,
    #[prop(into)] on_delete: Callback<String>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let guest_id = guest.id.clone();
    let guest_group_id = store_value(guest_group_id);
    let invitation_code = store_value(invitation_code);
    let (name, set_name) = create_signal(guest.name.clone());
    
    // Auto-focus the name input for new guests
    let input_ref = create_node_ref::<html::Input>();
    let is_new_guest = guest.name.is_empty() && guest.id.starts_with("temp_");
    
    create_effect(move |_| {
        if is_new_guest {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });
    let (vegetarian, set_vegetarian) = create_signal(guest.dietary_preferences.vegetarian);
    let (vegan, set_vegan) = create_signal(guest.dietary_preferences.vegan);
    let (halal, set_halal) = create_signal(guest.dietary_preferences.halal);
    let (no_pork, set_no_pork) = create_signal(guest.dietary_preferences.no_pork);
    let (gluten_free, set_gluten_free) = create_signal(guest.dietary_preferences.gluten_free);
    let (other, set_other) = create_signal(guest.dietary_preferences.other.clone());
    let (saving, set_saving) = create_signal(false);

    let save_changes = store_value({
        let guest_id = guest_id.clone();
        let is_temp = guest_id.starts_with("temp_");
        move || {
            let updated_guest = Guest {
                id: guest_id.clone(),
                guest_group_id: guest.guest_group_id.clone(),
                name: name.get(),
                dietary_preferences: DietaryPreferences {
                    vegetarian: vegetarian.get(),
                    vegan: vegan.get(),
                    halal: halal.get(),
                    no_pork: no_pork.get(),
                    gluten_free: gluten_free.get(),
                    other: other.get(),
                },
                created_at: None,
                updated_at: None,
            };

            if is_temp {
                // For temporary guests, just update local state
                on_update.call(updated_guest);
            } else {
                // For existing guests, update in database
                set_saving.set(true);

                let client = use_supabase_rpc();
                let guest_id = guest_id.clone();
                let group_id = guest_group_id.get_value();
                let inv_code = invitation_code.get_value();
                let guest_name = name.get();
                let dietary_prefs = updated_guest.dietary_preferences.clone();

                spawn_local(async move {
                    match client
                        .update_guest_secure(&guest_id, &group_id, &inv_code, &guest_name, &dietary_prefs)
                        .await
                    {
                        Ok(updated) => {
                            on_update.call(updated);
                            set_saving.set(false);
                        }
                        Err(_) => {
                            set_saving.set(false);
                        }
                    }
                });
            }
        }
    });

    let delete_guest = {
        let guest_id = guest_id.clone();
        let is_temp = guest_id.starts_with("temp_");
        move |_| {
            if is_temp {
                on_delete.call(guest_id.clone());
            } else {
                let client = use_supabase_rpc();
                let id = guest_id.clone();
                let group_id = guest_group_id.get_value();
                let inv_code = invitation_code.get_value();

                spawn_local(async move {
                    if client.delete_guest_secure(&id, &group_id, &inv_code).await.is_ok() {
                        on_delete.call(id);
                    }
                });
            }
        }
    };

    view! {
        <div class="bg-gray-50 p-3 sm:p-4 rounded-lg border border-gray-200 shadow-sm">
            <div class="space-y-3">
                <div class="flex items-start sm:items-center gap-2 w-full">
                    <input
                        node_ref=input_ref
                        type="text"
                        class="min-w-0 flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm sm:text-base font-semibold"
                        placeholder=move || translations().t("rsvp.invitee_name")
                        prop:value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        on:blur=move |_| save_changes.with_value(|f| f())
                    />
                    <button
                        class="flex-shrink-0 w-10 h-10 flex items-center justify-center text-red-600 hover:bg-red-100 rounded-lg transition-colors"
                        on:click=delete_guest
                        title=move || translations().t("rsvp.delete_invitee")
                    >
                        "🗑️"
                    </button>
                </div>

                <div class="pl-2">
                    <p class="text-xs font-semibold text-gray-600 mb-2">"Dietary Restrictions:"</p>
                    <div class="grid grid-cols-2 gap-2">
                        <label class="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                prop:checked=move || vegetarian.get()
                                on:change=move |ev| {
                                    set_vegetarian.set(event_target_checked(&ev));
                                    save_changes.with_value(|f| f());
                                }
                            />
                            <span class="text-xs text-gray-700">{move || translations().t("rsvp.vegetarian")}</span>
                        </label>

                        <label class="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                prop:checked=move || vegan.get()
                                on:change=move |ev| {
                                    set_vegan.set(event_target_checked(&ev));
                                    save_changes.with_value(|f| f());
                                }
                            />
                            <span class="text-xs text-gray-700">{move || translations().t("rsvp.vegan")}</span>
                        </label>

                        <label class="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                prop:checked=move || halal.get()
                                on:change=move |ev| {
                                    set_halal.set(event_target_checked(&ev));
                                    save_changes.with_value(|f| f());
                                }
                            />
                            <span class="text-xs text-gray-700">{move || translations().t("rsvp.halal")}</span>
                        </label>

                        <label class="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                prop:checked=move || no_pork.get()
                                on:change=move |ev| {
                                    set_no_pork.set(event_target_checked(&ev));
                                    save_changes.with_value(|f| f());
                                }
                            />
                            <span class="text-xs text-gray-700">{move || translations().t("rsvp.no_pork")}</span>
                        </label>

                        <label class="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                class="w-4 h-4 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                prop:checked=move || gluten_free.get()
                                on:change=move |ev| {
                                    set_gluten_free.set(event_target_checked(&ev));
                                    save_changes.with_value(|f| f());
                                }
                            />
                            <span class="text-xs text-gray-700">{move || translations().t("rsvp.gluten_free")}</span>
                        </label>
                    </div>

                    <input
                        type="text"
                        class="w-full mt-2 px-3 py-1.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent text-xs"
                        placeholder=move || translations().t("rsvp.other_dietary")
                        prop:value=move || other.get()
                        on:input=move |ev| set_other.set(event_target_value(&ev))
                        on:blur=move |_| save_changes.with_value(|f| f())
                    />
                </div>

                <Show when=move || saving.get()>
                    <div class="text-xs text-gray-500 italic">
                        {move || translations().t("common.saving")}
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn LocationAttendanceSection(
    guest_group: GuestGroup,
    location: Location,
    location_title: impl Fn() -> String + 'static + Copy,
    flag: &'static str,
    guests: ReadSignal<Vec<Guest>>,
    existing_rsvp: Option<Rsvp>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let location_key = store_value(format!("attending_{}", location.as_str()));
    
    // Track which guest IDs are attending this location
    let (attending_guest_ids, set_attending_guest_ids) = create_signal::<HashSet<String>>(HashSet::new());
    let (attending, set_attending) = create_signal(existing_rsvp.as_ref().map(|r| r.attending).unwrap_or(true));
    let (notes, set_notes) = create_signal(
        existing_rsvp
            .as_ref()
            .and_then(|r| r.additional_notes.clone())
            .unwrap_or_default(),
    );
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (success, set_success) = create_signal(false);
    let is_update = existing_rsvp.is_some();

    // Load saved attendance state from localStorage
    create_effect(move |_| {
        if let Some(storage) = window().local_storage().ok().flatten() {
            let key = location_key.get_value();
            if let Ok(Some(saved_ids)) = storage.get_item(&key) {
                if let Ok(ids) = serde_json::from_str::<Vec<String>>(&saved_ids) {
                    set_attending_guest_ids.set(ids.into_iter().collect());
                }
            }
        }
    });

    let toggle_guest = move |guest_id: String| {
        let mut current = attending_guest_ids.get_untracked();
        if current.contains(&guest_id) {
            current.remove(&guest_id);
        } else {
            current.insert(guest_id);
        }
        set_attending_guest_ids.set(current);
        
        // Save to localStorage
        if let Some(storage) = window().local_storage().ok().flatten() {
            let ids: Vec<String> = attending_guest_ids.get_untracked().into_iter().collect();
            if let Ok(json) = serde_json::to_string(&ids) {
                let key = location_key.get_value();
                let _ = storage.set_item(&key, &json);
            }
        }
    };

    let guest_group_id = guest_group.id.clone();
    let invitation_code = guest_group.invitation_code.clone();
    let original_party_size = guest_group.party_size;

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);
        set_success.set(false);

        let all_guests = guests.get();
        let attending_ids = attending_guest_ids.get();
        
        // Filter guests who are attending this location
        let attending_guests: Vec<_> = all_guests
            .iter()
            .filter(|g| attending_ids.contains(&g.id))
            .cloned()
            .collect();

        // Validate that all guests have names
        if attending.get() {
            let empty_names: Vec<_> = attending_guests
                .iter()
                .filter(|g| g.name.trim().is_empty())
                .collect();

            if !empty_names.is_empty() {
                set_loading.set(false);
                set_error.set(Some("Please fill in all guest names before submitting".to_string()));
                return;
            }

            if attending_guests.is_empty() {
                set_loading.set(false);
                set_error.set(Some("Please select at least one guest for this location".to_string()));
                return;
            }
        }

        let client = use_supabase_rpc();
        let group_id = guest_group_id.clone();
        let inv_code = invitation_code.clone();
        let location_str = location.as_str().to_string();
        let is_attending = attending.get();
        let guest_count = attending_guests.len() as i32;
        let additional_notes = if notes.get().is_empty() { None } else { Some(notes.get()) };

        spawn_local(async move {
            // Save all guests first (both temporary and existing)
            for guest in all_guests.iter() {
                if guest.id.starts_with("temp_") {
                    if let Err(e) = client
                        .create_guest_secure(&guest.guest_group_id, &inv_code, &guest.name, &guest.dietary_preferences)
                        .await
                    {
                        set_loading.set(false);
                        set_error.set(Some(format!("Error saving guest '{}': {}", guest.name, e)));
                        return;
                    }
                } else {
                    if let Err(e) = client
                        .update_guest_secure(&guest.id, &guest.guest_group_id, &inv_code, &guest.name, &guest.dietary_preferences)
                        .await
                    {
                        set_loading.set(false);
                        set_error.set(Some(format!("Error updating guest '{}': {}", guest.name, e)));
                        return;
                    }
                }
            }

            // Update party size if needed
            let total_guests = all_guests.len() as i32;
            if total_guests > original_party_size {
                if let Err(e) = client
                    .update_guest_group_party_size(&group_id, &inv_code, total_guests)
                    .await
                {
                    set_loading.set(false);
                    set_error.set(Some(format!("Error updating party size: {}", e)));
                    return;
                }
            }

            // Save RSVP
            match client
                .upsert_rsvp_secure(&group_id, &inv_code, &location_str, is_attending, guest_count, additional_notes)
                .await
            {
                Ok(_) => {
                    set_loading.set(false);
                    set_success.set(true);
                    set_error.set(None);
                }
                Err(e) => {
                    set_loading.set(false);
                    set_error.set(Some(format!("Error saving RSVP: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 animate-fade-in">
            <div class="flex items-center mb-6">
                <img src={flag} alt="Flag" class="w-14 h-10 sm:w-16 sm:h-12 mr-3 sm:mr-4 object-cover rounded shadow-md border border-gray-200"/>
                <h2 class="text-2xl sm:text-3xl font-serif font-bold text-gray-800">
                    {location_title}
                </h2>
            </div>

            <Show when=move || success.get()>
                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg mb-6 animate-fade-in">
                    <p class="font-semibold">{move || translations().t("rsvp.success")}</p>
                    <p class="text-sm mt-1">{move || translations().t("rsvp.success_thank_you")}</p>
                </div>
            </Show>

            <form on:submit=handle_submit class="space-y-6">
                <div>
                    <label class="block text-sm font-semibold text-gray-700 mb-3">
                        {move || translations().t("rsvp.attending")}
                    </label>
                    <div class="flex flex-col sm:flex-row gap-3 sm:gap-4">
                        <button
                            type="button"
                            class=move || {
                                let base = "flex-1 py-3 px-4 rounded-lg font-semibold transition-all duration-200 ";
                                if attending.get() {
                                    format!("{}bg-green-500 text-white shadow-md", base)
                                } else {
                                    format!("{}bg-gray-100 text-gray-700 hover:bg-gray-200", base)
                                }
                            }
                            on:click=move |_| set_attending.set(true)
                            disabled=move || loading.get()
                        >
                            "✓ " {move || translations().t("rsvp.yes")}
                        </button>
                        <button
                            type="button"
                            class=move || {
                                let base = "flex-1 py-3 px-4 rounded-lg font-semibold transition-all duration-200 ";
                                if !attending.get() {
                                    format!("{}bg-red-500 text-white shadow-md", base)
                                } else {
                                    format!("{}bg-gray-100 text-gray-700 hover:bg-gray-200", base)
                                }
                            }
                            on:click=move |_| set_attending.set(false)
                            disabled=move || loading.get()
                        >
                            "✗ " {move || translations().t("rsvp.no")}
                        </button>
                    </div>
                </div>

                <Show when=move || attending.get()>
                    <div class="space-y-4">
                        <div class="bg-gray-50 p-4 rounded-lg">
                            <h3 class="text-lg font-semibold text-gray-800 mb-3">
                                "Select Attending Guests"
                            </h3>
                            <p class="text-xs text-gray-600 mb-4">
                                "Choose which guests from your list will attend this location:"
                            </p>

                            <div class="space-y-2">
                                <For
                                    each=move || guests.get()
                                    key=|g| g.id.clone()
                                    children=move |guest: Guest| {
                                        let guest_id = store_value(guest.id.clone());
                                        let is_selected = move || attending_guest_ids.get().contains(&guest_id.get_value());
                                        
                                        // Make guest data reactive by looking it up from the signal using a memo
                                        let current_guest = create_memo(move |_| {
                                            guests.get()
                                                .into_iter()
                                                .find(|g| g.id == guest_id.get_value())
                                                .unwrap_or(guest.clone())
                                        });
                                        
                                        view! {
                                            <label class="flex items-center gap-3 p-3 bg-white rounded-lg border border-gray-200 hover:border-primary-300 cursor-pointer transition-colors">
                                                <input
                                                    type="checkbox"
                                                    class="w-5 h-5 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                                                    prop:checked=is_selected
                                                    on:change=move |_| toggle_guest(guest_id.get_value())
                                                />
                                                <div class="flex-1">
                                                    <div class="font-semibold text-gray-800">
                                                        {move || {
                                                            let g = current_guest.get();
                                                            if g.name.is_empty() { "(unnamed guest)".to_string() } else { g.name }
                                                        }}
                                                    </div>
                                                    {move || {
                                                        let g = current_guest.get();
                                                        let has_dietary = g.dietary_preferences.vegetarian
                                                            || g.dietary_preferences.vegan
                                                            || g.dietary_preferences.halal
                                                            || g.dietary_preferences.no_pork
                                                            || g.dietary_preferences.gluten_free
                                                            || !g.dietary_preferences.other.is_empty();
                                                        
                                                        if has_dietary {
                                                            let mut dietary_items = Vec::new();
                                                            if g.dietary_preferences.vegetarian {
                                                                dietary_items.push("🥗 Vegetarian");
                                                            }
                                                            if g.dietary_preferences.vegan {
                                                                dietary_items.push("🌱 Vegan");
                                                            }
                                                            if g.dietary_preferences.halal {
                                                                dietary_items.push("☪️ Halal");
                                                            }
                                                            if g.dietary_preferences.no_pork {
                                                                dietary_items.push("🚫🐷 No Pork");
                                                            }
                                                            if g.dietary_preferences.gluten_free {
                                                                dietary_items.push("🌾 Gluten-Free");
                                                            }
                                                            if !g.dietary_preferences.other.is_empty() {
                                                                dietary_items.push(g.dietary_preferences.other.as_str());
                                                            }
                                                            
                                                            view! {
                                                                <div class="text-xs text-gray-600 mt-1">
                                                                    {dietary_items.join(", ")}
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! {}.into_view()
                                                        }
                                                    }}
                                                </div>
                                            </label>
                                        }
                                    }
                                />
                            </div>

                            <div class="mt-3 text-sm text-gray-600">
                                <strong>{move || attending_guest_ids.get().len()}</strong>
                                " guest(s) selected"
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
                                prop:value=notes
                                on:input=move |ev| set_notes.set(event_target_value(&ev))
                                disabled=move || loading.get()
                            />
                        </div>
                    </div>
                </Show>

                <Show when=move || error.get().is_some()>
                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg animate-fade-in">
                        {move || error.get().unwrap_or_default()}
                    </div>
                </Show>

                <button
                    type="submit"
                    class=BUTTON_PRIMARY
                    disabled=move || loading.get()
                >
                    <Show
                        when=move || loading.get()
                        fallback=move || view! {
                            <span>
                                {move || if is_update {
                                    translations().t("rsvp.update")
                                } else {
                                    translations().t("rsvp.submit")
                                }}
                            </span>
                        }
                    >
                        <span class="flex items-center justify-center">
                            <svg class="animate-spin h-5 w-5 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            {move || translations().t("common.loading")}
                        </span>
                    </Show>
                </button>
            </form>
        </div>
    }
}