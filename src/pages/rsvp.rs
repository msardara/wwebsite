use crate::contexts::{use_supabase_rpc, GuestContext};
use crate::i18n::Translations;
use crate::styles::*;
use crate::types::{DietaryPreferences, Guest, GuestGroup, Language, Location, Rsvp, RsvpInput};
use leptos::*;
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
        <div class="max-w-3xl mx-auto px-4 sm:px-6">
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

                        match guest.location {
                            Location::Both => {
                                let sardinia_rsvp = rsvps.iter().find(|r| r.location == "sardinia").cloned();
                                let tunisia_rsvp = rsvps.iter().find(|r| r.location == "tunisia").cloned();

                                view! {
                                    <div>
                                        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 mb-6 sm:mb-8 animate-fade-in">
                                            <div class="text-center">
                                                <h2 class="text-2xl sm:text-3xl font-serif text-gray-800 mb-2">
                                                    {move || translations().t("rsvp.welcome")} ", "
                                                    <span class="text-primary-600">{guest.name.clone()}</span> "!"
                                                </h2>
                                                <p class="text-sm sm:text-base text-gray-600">
                                                    {move || translations().t("rsvp.both_events")}
                                                </p>
                                            </div>
                                        </div>

                                        <div class="space-y-8 sm:space-y-12">
                                            <LocationRsvpSection
                                                guest=guest.clone()
                                                location=Location::Sardinia
                                                location_title=move || translations().t("events.sardinia")
                                                flag="/public/sardinia-flag.png"
                                                existing_rsvp=sardinia_rsvp
                                                translations=translations
                                            />

                                            <LocationRsvpSection
                                                guest=guest.clone()
                                                location=Location::Tunisia
                                                location_title=move || translations().t("events.tunisia")
                                                flag="/public/tunisia-flag.png"
                                                existing_rsvp=tunisia_rsvp
                                                translations=translations
                                            />
                                        </div>
                                    </div>
                                }.into_view()
                            }
                            Location::Sardinia => {
                                let sardinia_rsvp = rsvps.iter().find(|r| r.location == "sardinia").cloned();
                                view! {
                                    <div>
                                        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 mb-6 sm:mb-8 animate-fade-in">
                                            <div class="text-center">
                                                <h2 class="text-2xl sm:text-3xl font-serif text-gray-800">
                                                    {move || translations().t("rsvp.welcome")} ", "
                                                    <span class="text-primary-600">{guest.name.clone()}</span> "!"
                                                </h2>
                                            </div>
                                        </div>

                                        <LocationRsvpSection
                                            guest=guest
                                            location=Location::Sardinia
                                            location_title=move || translations().t("events.sardinia")
                                            flag="/public/sardinia-flag.png"
                                            existing_rsvp=sardinia_rsvp
                                            translations=translations
                                        />
                                    </div>
                                }.into_view()
                            }
                            Location::Tunisia => {
                                let tunisia_rsvp = rsvps.iter().find(|r| r.location == "tunisia").cloned();
                                view! {
                                    <div>
                                        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 mb-6 sm:mb-8 animate-fade-in">
                                            <div class="text-center">
                                                <h2 class="text-2xl sm:text-3xl font-serif text-gray-800">
                                                    {move || translations().t("rsvp.welcome")} ", "
                                                    <span class="text-primary-600">{guest.name.clone()}</span> "!"
                                                </h2>
                                            </div>
                                        </div>

                                        <LocationRsvpSection
                                            guest=guest
                                            location=Location::Tunisia
                                            location_title=move || translations().t("events.tunisia")
                                            flag="/public/tunisia-flag.png"
                                            existing_rsvp=tunisia_rsvp
                                            translations=translations
                                        />
                                    </div>
                                }.into_view()
                            }
                        }
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
fn LocationRsvpSection(
    guest: GuestGroup,
    location: Location,
    location_title: impl Fn() -> String + 'static + Copy,
    flag: &'static str,
    existing_rsvp: Option<Rsvp>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let invitees_signal = create_rw_signal::<Vec<Guest>>(Vec::new());
    let guest_count = move || invitees_signal.get().len() as i32;

    view! {
        <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 animate-fade-in">
            <div class="flex items-center mb-6">
                <img src={flag} alt="Flag" class="w-14 h-10 sm:w-16 sm:h-12 mr-3 sm:mr-4 object-cover rounded shadow-md border border-gray-200"/>
                <h2 class="text-2xl sm:text-3xl font-serif font-bold text-gray-800">
                    {location_title}
                </h2>
            </div>

            <RsvpForm
                guest_group=guest.clone()
                location=location
                existing_rsvp=existing_rsvp
                translations=translations
                guest_count=guest_count
                guest_signal=invitees_signal
            />
        </div>
    }
}

#[component]
fn GuestManager(
    guest_group: GuestGroup,
    translations: impl Fn() -> Translations + 'static + Copy,
    invitees_signal: RwSignal<Vec<Guest>>,
) -> impl IntoView {
    let guest_group_id = store_value(guest_group.id.clone());
    let invitation_code = store_value(guest_group.invitation_code.clone());
    let (invitees, set_invitees) = (invitees_signal.read_only(), invitees_signal.write_only());
    let set_invitees_stored = store_value(set_invitees);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Clone guest_group_id and invitation_code for use in closures
    let guest_id_for_load = guest_group.id.clone();
    let invitation_code_for_load = guest_group.invitation_code.clone();
    let guest_group_id_for_add = guest_group.id.clone();

    // Load invitees on mount
    create_effect(move |_| {
        let client = use_supabase_rpc();
        let guest_id = guest_id_for_load.clone();
        let invitation_code = invitation_code_for_load.clone();

        spawn_local(async move {
            match client.get_guests(&guest_id, &invitation_code).await {
                Ok(invitees_list) => {
                    set_invitees.set(invitees_list);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Error loading invitees: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    // Allow unlimited guests to be added

    let guest_group_id_stored = store_value(guest_group_id_for_add);
    let add_invitee = move |_| {
        let guest_group_id = guest_group_id_stored.get_value();
        // Create a temporary guest with empty name - will be saved on RSVP submit
        let temp_id = format!("temp_{}", Uuid::new_v4());
        let new_invitee = Guest {
            id: temp_id,
            guest_group_id,
            name: String::new(),
            dietary_preferences: DietaryPreferences::default(),
            created_at: None,
            updated_at: None,
        };

        set_invitees_stored.with_value(|set_inv| {
            set_inv.update(|list| list.push(new_invitee));
        });
    };

    view! {
        <div class="mb-6 p-4 sm:p-6 bg-gray-50 rounded-lg">
            <h3 class="text-xl sm:text-2xl font-serif font-bold text-gray-800 mb-4">
                {move || translations().t("rsvp.invitees_title")}
            </h3>

            <Show when=move || error.get().is_some()>
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg mb-4">
                    {move || error.get().unwrap_or_default()}
                </div>
            </Show>

            <Show
                when=move || !loading.get()
                fallback=move || view! {
                    <div class="text-gray-600">{move || translations().t("common.loading")}</div>
                }
            >
                <div class="space-y-3 mb-4">
                    <For
                        each=move || invitees.get()
                        key=|invitee| invitee.id.clone()
                        children=move |invitee: Guest| {
                            let on_update = Callback::new(move |updated: Guest| {
                                set_invitees_stored.with_value(|set_inv| {
                                    set_inv.update(|list| {
                                        if let Some(pos) = list.iter().position(|i| i.id == updated.id) {
                                            list[pos] = updated;
                                        }
                                    });
                                });
                            });
                            let on_delete = Callback::new(move |id: String| {
                                set_invitees_stored.with_value(|set_inv| {
                                    set_inv.update(|list| {
                                        list.retain(|i| i.id != id);
                                    });
                                });
                            });
                            view! {
                                <InviteeCard
                                    invitee=invitee.clone()
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
                        {move || invitees.get().len()} " " {move || if invitees.get().len() == 1 { "guest" } else { "guests" }}
                    </span>
                    <button
                        class="w-full sm:w-auto px-4 py-2 bg-secondary-500 text-gray-900 rounded-lg hover:bg-secondary-600 transition-all font-semibold shadow-md border-2 border-secondary-700"
                        on:click=add_invitee
                    >
                        "+ " {move || translations().t("rsvp.add_invitee")}
                    </button>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn InviteeCard(
    invitee: Guest,
    guest_group_id: String,
    invitation_code: String,
    #[prop(into)] on_update: Callback<Guest>,
    #[prop(into)] on_delete: Callback<String>,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let invitee_id = invitee.id.clone();
    let guest_group_id_for_save = guest_group_id.clone();
    let invitation_code_for_save = invitation_code.clone();
    let guest_group_id_for_delete = guest_group_id.clone();
    let invitation_code_for_delete = invitation_code.clone();
    let (name, set_name) = create_signal(invitee.name.clone());
    let (vegetarian, set_vegetarian) = create_signal(invitee.dietary_preferences.vegetarian);
    let (vegan, set_vegan) = create_signal(invitee.dietary_preferences.vegan);
    let (gluten_free, set_gluten_free) = create_signal(invitee.dietary_preferences.gluten_free);
    let (other, set_other) = create_signal(invitee.dietary_preferences.other.clone());
    let (saving, set_saving) = create_signal(false);

    let save_changes = store_value({
        let invitee_id = invitee_id.clone();
        let is_temp = invitee_id.starts_with("temp_");
        move || {
            // For temporary guests, just update the local state without database call
            if is_temp {
                let updated_invitee = Guest {
                    id: invitee_id.clone(),
                    guest_group_id: invitee.guest_group_id.clone(),
                    name: name.get(),
                    dietary_preferences: DietaryPreferences {
                        vegetarian: vegetarian.get(),
                        vegan: vegan.get(),
                        gluten_free: gluten_free.get(),
                        other: other.get(),
                    },
                    created_at: None,
                    updated_at: None,
                };
                on_update.call(updated_invitee);
            } else {
                // For existing guests, update in database
                set_saving.set(true);

                let dietary_prefs = DietaryPreferences {
                    vegetarian: vegetarian.get(),
                    vegan: vegan.get(),
                    gluten_free: gluten_free.get(),
                    other: other.get(),
                };

                let client = use_supabase_rpc();
                let invitee_id = invitee_id.clone();
                let guest_group_id = guest_group_id_for_save.clone();
                let invitation_code = invitation_code_for_save.clone();
                let guest_name = name.get();

                spawn_local(async move {
                    match client
                        .update_guest_secure(
                            &invitee_id,
                            &guest_group_id,
                            &invitation_code,
                            &guest_name,
                            &dietary_prefs,
                        )
                        .await
                    {
                        Ok(updated_invitee) => {
                            on_update.call(updated_invitee);
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

    let delete_invitee = {
        let invitee_id = invitee_id.clone();
        let guest_group_id_clone = guest_group_id_for_delete.clone();
        let invitation_code_clone = invitation_code_for_delete.clone();
        let is_temp = invitee_id.starts_with("temp_");
        move |_| {
            // For temporary guests, just remove from local state
            if is_temp {
                on_delete.call(invitee_id.clone());
            } else {
                // For existing guests, delete from database
                let client = use_supabase_rpc();
                let invitee_id = invitee_id.clone();
                let guest_group_id = guest_group_id_clone.clone();
                let invitation_code = invitation_code_clone.clone();
                let id_for_callback = invitee_id.clone();

                spawn_local(async move {
                    if client
                        .delete_guest_secure(&invitee_id, &guest_group_id, &invitation_code)
                        .await
                        .is_ok()
                    {
                        on_delete.call(id_for_callback);
                    }
                });
            }
        }
    };

    view! {
        <div class="bg-white p-3 sm:p-4 rounded-lg border border-gray-200 shadow-sm overflow-hidden">
            <div class="space-y-3">
                <div class="flex items-start sm:items-center gap-2 w-full">
                    <input
                        type="text"
                        class="min-w-0 flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm sm:text-base"
                        placeholder=move || translations().t("rsvp.invitee_name")
                        prop:value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        on:blur=move |_| save_changes.with_value(|f| f())
                    />
                    <button
                        class="flex-shrink-0 w-10 h-10 flex items-center justify-center text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                        on:click=delete_invitee
                        title=move || translations().t("rsvp.delete_invitee")
                    >
                        "🗑️"
                    </button>
                </div>

                <div class="space-y-2.5">
                    <label class="flex items-start sm:items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="mt-0.5 sm:mt-0 w-4 h-4 flex-shrink-0 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                            prop:checked=move || vegetarian.get()
                            on:change=move |ev| {
                                set_vegetarian.set(event_target_checked(&ev));
                                save_changes.with_value(|f| f());
                            }
                        />
                        <span class="text-sm text-gray-700">{move || translations().t("rsvp.vegetarian")}</span>
                    </label>

                    <label class="flex items-start sm:items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="mt-0.5 sm:mt-0 w-4 h-4 flex-shrink-0 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                            prop:checked=move || vegan.get()
                            on:change=move |ev| {
                                set_vegan.set(event_target_checked(&ev));
                                save_changes.with_value(|f| f());
                            }
                        />
                        <span class="text-sm text-gray-700">{move || translations().t("rsvp.vegan")}</span>
                    </label>

                    <label class="flex items-start sm:items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            class="mt-0.5 sm:mt-0 w-4 h-4 flex-shrink-0 text-primary-600 rounded focus:ring-2 focus:ring-primary-500"
                            prop:checked=move || gluten_free.get()
                            on:change=move |ev| {
                                set_gluten_free.set(event_target_checked(&ev));
                                save_changes.with_value(|f| f());
                            }
                        />
                        <span class="text-sm text-gray-700">{move || translations().t("rsvp.gluten_free")}</span>
                    </label>

                    <input
                        type="text"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent text-sm"
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
fn RsvpForm(
    guest_group: GuestGroup,
    location: Location,
    existing_rsvp: Option<Rsvp>,
    translations: impl Fn() -> Translations + 'static + Copy,
    guest_count: impl Fn() -> i32 + 'static + Copy,
    guest_signal: RwSignal<Vec<Guest>>,
) -> impl IntoView {
    let is_update = existing_rsvp.is_some();

    // Store guest_id and invitation_code to avoid move issues
    let guest_group_id = guest_group.id.clone();
    let invitation_code = store_value(guest_group.invitation_code.clone());
    let original_party_size = guest_group.party_size;

    let (attending, set_attending) =
        create_signal(existing_rsvp.as_ref().map(|r| r.attending).unwrap_or(true));
    let (notes, set_notes) = create_signal(
        existing_rsvp
            .as_ref()
            .and_then(|r| r.additional_notes.clone())
            .unwrap_or_default(),
    );

    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (success, set_success) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        set_loading.set(true);
        set_error.set(None);
        set_success.set(false);

        // Get all guests
        let guests = guest_signal.get();

        // Validate that all guests have names
        let empty_names: Vec<_> = guests
            .iter()
            .enumerate()
            .filter(|(_, g)| g.name.trim().is_empty())
            .collect();

        if !empty_names.is_empty() {
            set_loading.set(false);
            set_error.set(Some(
                "Please fill in all guest names before submitting".to_string(),
            ));
            return;
        }

        // Note: Dietary preferences are saved per guest in the guests table
        // They are not stored in the RSVP record
        let rsvp_input = RsvpInput {
            guest_group_id: guest_group_id.clone(),
            location: location.as_str().to_string(),
            attending: attending.get(),
            number_of_guests: guest_count(),
            additional_notes: if notes.get().is_empty() {
                None
            } else {
                Some(notes.get())
            },
        };

        let client = use_supabase_rpc();
        let guest_id_for_request = guest_group_id.clone();
        let location_str = location.as_str().to_string();
        let current_guest_count = guests.len() as i32;
        let invitation_code_value = invitation_code.get_value();

        spawn_local(async move {
            // First, save all guests (new and updated)
            for guest in guests.iter() {
                // If it's a temporary guest (starts with "temp_"), create it
                if guest.id.starts_with("temp_") {
                    if let Err(e) = client
                        .create_guest_secure(
                            &guest.guest_group_id,
                            &invitation_code_value,
                            &guest.name,
                            &guest.dietary_preferences,
                        )
                        .await
                    {
                        set_loading.set(false);
                        set_error.set(Some(format!("Error saving guest '{}': {}", guest.name, e)));
                        return;
                    }
                } else {
                    // Update existing guest
                    if let Err(e) = client
                        .update_guest_secure(
                            &guest.id,
                            &guest.guest_group_id,
                            &invitation_code_value,
                            &guest.name,
                            &guest.dietary_preferences,
                        )
                        .await
                    {
                        set_loading.set(false);
                        set_error.set(Some(format!(
                            "Error updating guest '{}': {}",
                            guest.name, e
                        )));
                        return;
                    }
                }
            }

            // Update party_size if more guests were added than originally expected
            // Use secure method that validates invitation code
            if current_guest_count > original_party_size {
                if let Err(e) = client
                    .update_guest_group_party_size(
                        &guest_id_for_request,
                        &invitation_code_value,
                        current_guest_count,
                    )
                    .await
                {
                    set_loading.set(false);
                    set_error.set(Some(format!("Error updating party size: {}", e)));
                    return;
                }
            }

            // Then save the RSVP using secure upsert
            // Note: Dietary preferences are saved per guest, not in the RSVP
            let result = client
                .upsert_rsvp_secure(
                    &guest_id_for_request,
                    &invitation_code_value,
                    &location_str,
                    rsvp_input.attending,
                    rsvp_input.number_of_guests,
                    rsvp_input.additional_notes,
                )
                .await;

            match result {
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
        <div>
            <Show when=move || success.get()>
                <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg mb-6 animate-fade-in">
                    <p class="font-semibold">{move || translations().t("rsvp.success")}</p>
                    <p class="text-sm mt-1">{move || translations().t("rsvp.success_thank_you")}</p>
                </div>
            </Show>

            {/* Attending */}
            <div class="mb-6">
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

            {/* Show guest list and additional notes only if attending */}
            <Show when=move || attending.get()>
                <div class="bg-white rounded-lg shadow-lg p-4 sm:p-6 lg:p-8 space-y-6 animate-fade-in">
                    <GuestManager
                        guest_group=guest_group.clone()
                        translations=translations
                        invitees_signal=guest_signal
                    />

                    {/* Additional notes */}
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

            <form on:submit=handle_submit class="space-y-6">

                <Show when=move || error.get().is_some()>
                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg animate-fade-in">
                        {move || error.get().unwrap_or_default()}
                    </div>
                </Show>

                {/* Action buttons */}
                <div>
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
                </div>
            </form>
        </div>
    }
}
