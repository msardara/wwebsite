//! Guest group add/edit modal component.
//!
//! Extracted from `guests.rs` for better modularity.

use crate::contexts::AdminContext;
use crate::types::{DietaryPreferences, GuestGroup, GuestGroupInput, GuestGroupUpdate, GuestInput};
use leptos::*;
use web_sys::console;

#[derive(Debug, Clone)]
pub(super) struct GuestRow {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub age_category: crate::types::AgeCategory,
}

impl GuestRow {
    fn new(id: usize) -> Self {
        Self {
            id,
            first_name: String::new(),
            last_name: String::new(),
            age_category: crate::types::AgeCategory::default(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
            .trim()
            .to_string()
    }
}

#[component]
pub fn GuestgroupModal(
    guest: Option<GuestGroup>,
    on_close: impl Fn() + 'static,
    on_save: impl Fn() + 'static,
) -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let is_edit = guest.is_some();
    let (name, set_name) = create_signal(guest.as_ref().map_or(String::new(), |g| g.name.clone()));
    let (email, set_email) = create_signal(
        guest
            .as_ref()
            .and_then(|g| g.email.clone())
            .unwrap_or_default(),
    );

    let (_party_size, _set_party_size) = create_signal(guest.as_ref().map_or(1, |g| g.party_size));
    // Parse initial location(s) from Vec<String>
    let initial_locations = guest
        .as_ref()
        .map(|g| g.locations.clone())
        .unwrap_or_else(|| vec!["sardinia".to_string()]);

    let (sardinia_selected, set_sardinia_selected) =
        create_signal(initial_locations.contains(&"sardinia".to_string()));
    let (tunisia_selected, set_tunisia_selected) =
        create_signal(initial_locations.contains(&"tunisia".to_string()));
    let (nice_selected, set_nice_selected) =
        create_signal(initial_locations.contains(&"nice".to_string()));

    let initial_invited_by = guest
        .as_ref()
        .map(|g| g.invited_by.clone())
        .unwrap_or_default();
    let (mauro_selected, set_mauro_selected) =
        create_signal(initial_invited_by.contains(&"mauro.sardara@gmail.com".to_string()));
    let (muna_selected, set_muna_selected) =
        create_signal(initial_invited_by.contains(&"munaamamu0@gmail.com".to_string()));

    let (default_language, set_default_language) = create_signal(
        guest
            .as_ref()
            .map_or("en".to_string(), |g| g.default_language.clone()),
    );

    // Compute locations array from checkboxes
    let get_locations_value = move || -> Vec<String> {
        let mut locs = Vec::new();
        if sardinia_selected.get() {
            locs.push("sardinia".to_string());
        }
        if tunisia_selected.get() {
            locs.push("tunisia".to_string());
        }
        if nice_selected.get() {
            locs.push("nice".to_string());
        }
        if locs.is_empty() {
            locs.push("sardinia".to_string()); // Default
        }
        locs
    };

    let get_invited_by_value = move || -> Vec<String> {
        let mut admins = Vec::new();
        if mauro_selected.get() {
            admins.push("mauro.sardara@gmail.com".to_string());
        }
        if muna_selected.get() {
            admins.push("munaamamu0@gmail.com".to_string());
        }
        admins
    };

    let language_select_ref = create_node_ref::<html::Select>();

    // Set the select value after the element is mounted
    create_effect(move |_| {
        if let Some(select_el) = language_select_ref.get() {
            select_el.set_value(&default_language.get());
        }
    });

    // Dynamic invitee list
    let (invitees, set_invitees) = create_signal::<Vec<GuestRow>>(vec![GuestRow::new(0)]);
    let (next_id, set_next_id) = create_signal(1_usize);
    let (loading_guests, set_loading_guests) = create_signal(is_edit);

    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let guest_id = guest.as_ref().map(|g| g.id.clone());

    // Load existing guests when editing
    if let Some(ref group) = guest {
        let group_id = group.id.clone();
        let admin_ctx = admin_context;
        spawn_local(async move {
            match admin_ctx
                .authenticated_client()
                .get_guests_admin(&group_id)
                .await
            {
                Ok(guests_list) => {
                    let mut loaded_invitees = Vec::new();
                    let mut max_id = 0;
                    for (idx, guest) in guests_list.into_iter().enumerate() {
                        let id = idx;
                        // Split name into first and last
                        let parts: Vec<&str> = guest.name.split_whitespace().collect();
                        let (first, last) = if parts.len() > 1 {
                            (parts[0].to_string(), parts[1..].join(" "))
                        } else {
                            (guest.name.clone(), String::new())
                        };
                        loaded_invitees.push(GuestRow {
                            id,
                            first_name: first,
                            last_name: last,
                            age_category: guest.age_category.clone(),
                        });
                        max_id = id + 1;
                    }
                    if !loaded_invitees.is_empty() {
                        set_invitees.set(loaded_invitees);
                        set_next_id.set(max_id);
                    }
                    set_loading_guests.set(false);
                }
                Err(e) => {
                    console::error_1(&format!("Failed to load guests: {:?}", e).into());
                    set_loading_guests.set(false);
                }
            }
        });
    }

    let on_close = store_value(on_close);
    let on_save = store_value(on_save);

    // Update invitee first name
    let update_invitee_first_name = move |id: usize, value: String| {
        set_invitees.update(|list| {
            if let Some(invitee) = list.iter_mut().find(|i| i.id == id) {
                invitee.first_name = value;
            }
        });
    };

    // Update invitee last name
    let update_invitee_last_name = move |id: usize, value: String| {
        set_invitees.update(|list| {
            if let Some(invitee) = list.iter_mut().find(|i| i.id == id) {
                invitee.last_name = value;
            }
        });
    };

    // Update invitee age category
    let update_invitee_age_category = move |id: usize, age_cat: crate::types::AgeCategory| {
        set_invitees.update(|list| {
            if let Some(invitee) = list.iter_mut().find(|i| i.id == id) {
                invitee.age_category = age_cat;
            }
        });
    };

    // Add new invitee row
    let add_invitee = move |_| {
        let id = next_id.get();
        set_next_id.set(id + 1);
        set_invitees.update(|list| list.push(GuestRow::new(id)));
    };

    // Remove invitee row
    let remove_invitee = move |id: usize| {
        set_invitees.update(|list| list.retain(|i| i.id != id));
    };

    let handle_save = {
        move |_| {
            let guest_id = guest_id.clone();
            spawn_local(async move {
                set_saving.set(true);
                set_error.set(None);

                let result = if let Some(id) = guest_id {
                    // Update existing guest group
                    let invitees_list = invitees.get();
                    let valid_invitees: Vec<GuestRow> = invitees_list
                        .into_iter()
                        .filter(|i| !i.first_name.is_empty() || !i.last_name.is_empty())
                        .collect();

                    if valid_invitees.is_empty() {
                        set_error.set(Some(
                            "Please add at least one guest with a name".to_string(),
                        ));
                        set_saving.set(false);
                        return;
                    }

                    let actual_party_size = valid_invitees.len() as i32;

                    let update = GuestGroupUpdate {
                        name: Some(name.get()),
                        email: if email.get().is_empty() {
                            None
                        } else {
                            Some(email.get())
                        },
                        party_size: Some(actual_party_size),
                        locations: Some(get_locations_value()),
                        default_language: Some(default_language.get()),
                        additional_notes: None,
                        invitation_sent: None,
                        invited_by: Some(get_invited_by_value()),
                    };

                    // Update guest group
                    let group_result = admin_context
                        .authenticated_client()
                        .update_guest_group(&id, &update)
                        .await;

                    if group_result.is_ok() {
                        // Delete all existing guests and recreate them
                        let client = admin_context.authenticated_client();

                        // Get existing guests
                        if let Ok(existing_guests) = client.get_guests_admin(&id).await {
                            // Delete all existing guests
                            for guest in existing_guests {
                                let _ = client.delete_guest(&guest.id).await;
                            }
                        }

                        // Create new guests
                        for invitee_row in valid_invitees {
                            let guest_input = GuestInput {
                                guest_group_id: id.clone(),
                                name: invitee_row.full_name(),
                                attending_locations: vec![],
                                dietary_preferences: DietaryPreferences::default(),
                                age_category: invitee_row.age_category.clone(),
                            };

                            if let Err(e) = client.create_guest(&guest_input).await {
                                console::error_1(
                                    &format!("Failed to create guest: {:?}", e).into(),
                                );
                            }
                        }
                    }

                    group_result
                } else {
                    // Create new guest with invitees
                    let invitees_list = invitees.get();
                    let valid_invitees: Vec<GuestRow> = invitees_list
                        .into_iter()
                        .filter(|i| !i.first_name.is_empty() || !i.last_name.is_empty())
                        .collect();

                    if valid_invitees.is_empty() {
                        set_error.set(Some(
                            "Please add at least one guest with a name".to_string(),
                        ));
                        set_saving.set(false);
                        return;
                    }

                    let actual_party_size = valid_invitees.len() as i32;

                    let input = GuestGroupInput {
                        name: name.get(),
                        email: if email.get().is_empty() {
                            None
                        } else {
                            Some(email.get())
                        },
                        party_size: actual_party_size,
                        locations: get_locations_value(),
                        default_language: default_language.get(),
                        additional_notes: None,
                        invited_by: get_invited_by_value(),
                    };

                    // Create the guest group
                    match admin_context
                        .authenticated_client()
                        .create_guest_group(&input)
                        .await
                    {
                        Ok(created_group) => {
                            console::log_1(&format!("Guest created: {}", created_group.id).into());

                            // Create invitees for this guest
                            let client = admin_context.authenticated_client();
                            for invitee_row in valid_invitees {
                                let guest_input = GuestInput {
                                    guest_group_id: created_group.id.clone(),
                                    name: invitee_row.full_name(),
                                    attending_locations: vec![],
                                    dietary_preferences: DietaryPreferences::default(),
                                    age_category: invitee_row.age_category.clone(),
                                };

                                console::log_1(
                                    &format!("Creating invitee: {}", guest_input.name).into(),
                                );

                                // Create invitee and log any errors
                                match client.create_guest(&guest_input).await {
                                    Ok(invitee) => {
                                        console::log_1(
                                            &format!("Invitee created: {}", invitee.name).into(),
                                        );
                                    }
                                    Err(e) => {
                                        console::error_1(
                                            &format!("Failed to create invitee: {:?}", e).into(),
                                        );
                                    }
                                }
                            }

                            Ok(created_group)
                        }
                        Err(e) => Err(e),
                    }
                };

                match result {
                    Ok(_) => {
                        set_saving.set(false);
                        on_save.with_value(|f| f());
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to save guest: {}", e)));
                        set_saving.set(false);
                    }
                }
            });
        }
    };

    view! {
        <div class="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50 p-4 backdrop-blur-sm">
            <div class="bg-white rounded-2xl shadow-2xl max-w-3xl w-full max-h-[90vh] overflow-hidden border-2 border-secondary-200">
                <div class="bg-white px-6 py-5 flex items-center justify-between border-b-4 border-secondary-500">
                    <div class="flex items-center gap-3">
                        <div class="w-12 h-12 bg-secondary-600 rounded-full flex items-center justify-center">
                            <span class="text-3xl">{if is_edit { "✏️" } else { "➕" }}</span>
                        </div>
                        <h3 class="text-2xl font-bold text-gray-900">
                            {if is_edit { "Edit Guest Group" } else { "Add New Guest Group" }}
                        </h3>
                    </div>
                    <button
                        on:click=move |_| on_close.with_value(|f| f())
                        class="text-gray-600 hover:bg-gray-100 rounded-lg p-2 transition-all text-2xl font-bold"
                    >
                        "✕"
                    </button>
                </div>

                <div class="p-6 overflow-y-auto max-h-[calc(90vh-80px)]">
                    {move || error.get().map(|err| view! {
                        <div class="mb-6 bg-red-50 border-l-4 border-red-500 text-red-800 px-4 py-4 rounded-r-lg shadow-sm flex items-start gap-3">
                            <span class="text-2xl flex-shrink-0">"❌"</span>
                            <div>
                                <p class="font-semibold mb-1">"Error"</p>
                                <p class="text-sm">{err}</p>
                            </div>
                        </div>
                    })}

                    <form on:submit=|e| e.prevent_default() class="space-y-5">
                        <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
                            <h4 class="text-sm font-bold text-gray-700 mb-4 flex items-center gap-2">
                                <span>"📋"</span>
                                <span>"Group Information"</span>
                            </h4>
                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Group Name" <span class="text-red-500">"*"</span>
                                    </label>
                                    <input
                                        type="text"
                                        required
                                        placeholder="e.g., Smith Family"
                                        class="w-full px-4 py-3 border-2 border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 transition-all font-medium"
                                        prop:value=move || name.get()
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                    />
                                </div>

                                <div>
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Contact Email"
                                    </label>
                                    <input
                                        type="email"
                                        placeholder="contact@example.com"
                                        class="w-full px-4 py-3 border-2 border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 transition-all font-medium"
                                        prop:value=move || email.get()
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>
                        </div>



                        <div>
                            <label class="block text-base font-semibold text-gray-800 mb-3">
                                "Guest List" <span class="text-red-500">"*"</span>
                            </label>

                            <Show
                                when=move || !loading_guests.get()
                                fallback=move || view! {
                                    <div class="bg-gray-50 rounded-xl p-8 text-center">
                                        <div class="animate-spin text-4xl mb-2">"⏳"</div>
                                        <p class="text-gray-600">"Loading guests..."</p>
                                    </div>
                                }
                            >
                            <div class="bg-gradient-to-br from-gray-50 to-secondary-50 rounded-xl p-4 space-y-3 max-h-80 overflow-y-auto border-2 border-secondary-200 shadow-inner">
                                <For
                                    each=move || invitees.get()
                                    key=|invitee| invitee.id
                                    children=move |invitee_row| {
                                        let invitee_id = invitee_row.id;
                                        let first_name_value = invitee_row.first_name.clone();
                                        let last_name_value = invitee_row.last_name.clone();
                                        let age_cat = store_value(invitee_row.age_category.clone());
                                        let can_remove = invitees.get().len() > 1;

                                        view! {
                                            <div class="bg-white p-3 rounded-lg border-2 border-gray-200 hover:border-secondary-300 transition-colors shadow-sm">
                                                <div class="flex items-center gap-2 mb-2">
                                                <div class="flex-shrink-0 w-8 h-8 bg-secondary-100 text-secondary-700 rounded-full flex items-center justify-center font-bold text-xs">
                                                    {invitee_id + 1}
                                                </div>
                                                <input
                                                    type="text"
                                                    placeholder="First Name"
                                                    class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 text-sm font-medium"
                                                    value=first_name_value
                                                    on:input=move |ev| {
                                                        update_invitee_first_name(invitee_id, event_target_value(&ev));
                                                    }
                                                />
                                                <input
                                                    type="text"
                                                    placeholder="Last Name"
                                                    class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 text-sm font-medium"
                                                    value=last_name_value
                                                    on:input=move |ev| {
                                                        update_invitee_last_name(invitee_id, event_target_value(&ev));
                                                    }
                                                />
                                                {if can_remove {
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| remove_invitee(invitee_id)
                                                        class="flex-shrink-0 w-8 h-8 flex items-center justify-center text-red-600 hover:text-white hover:bg-red-500 rounded-lg transition-all font-bold"
                                                        title="Remove guest"
                                                    >
                                                        "✕"
                                                    </button>
                                                }.into_view()
                                            } else {
                                                view! { <div class="w-8"></div> }.into_view()
                                            }}
                                            </div>

                                            {/* Age Category Selection */}
                                            <div class="flex items-center gap-2 pl-10">
                                                <span class="text-xs font-semibold text-gray-600 mr-2">"Age:"</span>
                                                <label class="flex items-center gap-1 cursor-pointer">
                                                    <input
                                                        type="radio"
                                                        name={format!("age_cat_{}", invitee_id)}
                                                        class="w-3 h-3 text-secondary-600"
                                                        prop:checked=move || age_cat.get_value().as_str() == "adult"
                                                        on:change=move |_| {
                                                            update_invitee_age_category(invitee_id, crate::types::AgeCategory::Adult);
                                                        }
                                                    />
                                                    <span class="text-xs text-gray-700">"Adult"</span>
                                                </label>
                                                <label class="flex items-center gap-1 cursor-pointer">
                                                    <input
                                                        type="radio"
                                                        name={format!("age_cat_{}", invitee_id)}
                                                        class="w-3 h-3 text-secondary-600"
                                                        prop:checked=move || age_cat.get_value().as_str() == "child_under_3"
                                                        on:change=move |_| {
                                                            update_invitee_age_category(invitee_id, crate::types::AgeCategory::ChildUnder3);
                                                        }
                                                    />
                                                    <span class="text-xs text-gray-700">"< 3yo"</span>
                                                </label>
                                                <label class="flex items-center gap-1 cursor-pointer">
                                                    <input
                                                        type="radio"
                                                        name={format!("age_cat_{}", invitee_id)}
                                                        class="w-3 h-3 text-secondary-600"
                                                        prop:checked=move || age_cat.get_value().as_str() == "child_under_10"
                                                        on:change=move |_| {
                                                            update_invitee_age_category(invitee_id, crate::types::AgeCategory::ChildUnder10);
                                                        }
                                                    />
                                                    <span class="text-xs text-gray-700">"< 10yo"</span>
                                                </label>
                                            </div>
                                        </div>
                                    }
                                }
                                />

                                <button
                                    type="button"
                                    on:click=add_invitee
                                    class="w-full px-4 py-3 bg-secondary-500 text-gray-900 rounded-lg hover:bg-secondary-600 transition-all duration-200 flex items-center justify-center space-x-2 font-bold shadow-md border-2 border-secondary-700 hover:shadow-lg"
                                >
                                    <span class="text-xl">"+"</span>
                                    <span>"Add Another Guest"</span>
                                </button>
                            </div>
                            </Show>
                        </div>

                        <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
                            <label class="block text-sm font-semibold text-gray-700 mb-3">
                                "Wedding Location(s)" <span class="text-red-500">"*"</span>
                            </label>
                            <p class="text-xs text-gray-600 mb-3">"Select which location(s) this group is invited to:"</p>
                            <div class="space-y-2">
                                <label class="flex items-center gap-3 p-3 bg-white rounded-lg border-2 border-gray-300 hover:border-secondary-500 cursor-pointer transition-colors">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-secondary-600 rounded focus:ring-2 focus:ring-secondary-500"
                                        prop:checked=move || sardinia_selected.get()
                                        on:change=move |ev| set_sardinia_selected.set(event_target_checked(&ev))
                                    />
                                    <span class="font-semibold text-gray-800">"🏖️ Sardinia"</span>
                                </label>
                                <label class="flex items-center gap-3 p-3 bg-white rounded-lg border-2 border-gray-300 hover:border-secondary-500 cursor-pointer transition-colors">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-secondary-600 rounded focus:ring-2 focus:ring-secondary-500"
                                        prop:checked=move || tunisia_selected.get()
                                        on:change=move |ev| set_tunisia_selected.set(event_target_checked(&ev))
                                    />
                                    <span class="font-semibold text-gray-800">"🌴 Tunisia"</span>
                                </label>
                                <label class="flex items-center gap-3 p-4 border-2 border-gray-200 rounded-lg hover:border-secondary-400 hover:bg-secondary-50 transition-all cursor-pointer">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-secondary-600 rounded focus:ring-2 focus:ring-secondary-500"
                                        prop:checked=move || nice_selected.get()
                                        on:change=move |ev| set_nice_selected.set(event_target_checked(&ev))
                                    />
                                    <span class="font-semibold text-gray-800">"🇫🇷 Nice"</span>
                                </label>
                            </div>
                        </div>

                        <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
                            <label class="block text-sm font-semibold text-gray-700 mb-3">
                                "Invited By"
                            </label>
                            <p class="text-xs text-gray-600 mb-3">"Which admin(s) invited this group:"</p>
                            <div class="space-y-2">
                                <label class="flex items-center gap-3 p-3 bg-white rounded-lg border-2 border-gray-300 hover:border-secondary-500 cursor-pointer transition-colors">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-secondary-600 rounded focus:ring-2 focus:ring-secondary-500"
                                        prop:checked=move || mauro_selected.get()
                                        on:change=move |ev| set_mauro_selected.set(event_target_checked(&ev))
                                    />
                                    <span class="font-semibold text-gray-800">"Mauro"</span>
                                    <span class="text-xs text-gray-500">"(mauro.sardara@gmail.com)"</span>
                                </label>
                                <label class="flex items-center gap-3 p-3 bg-white rounded-lg border-2 border-gray-300 hover:border-secondary-500 cursor-pointer transition-colors">
                                    <input
                                        type="checkbox"
                                        class="w-5 h-5 text-secondary-600 rounded focus:ring-2 focus:ring-secondary-500"
                                        prop:checked=move || muna_selected.get()
                                        on:change=move |ev| set_muna_selected.set(event_target_checked(&ev))
                                    />
                                    <span class="font-semibold text-gray-800">"Muna"</span>
                                    <span class="text-xs text-gray-500">"(munaamamu0@gmail.com)"</span>
                                </label>
                            </div>
                        </div>

                        <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
                            <label class="block text-sm font-semibold text-gray-700 mb-2">
                                "Default Language" <span class="text-red-500">"*"</span>
                            </label>
                            <select
                                node_ref=language_select_ref
                                required
                                class="w-full px-4 py-3 border-2 border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 font-semibold text-base bg-white"
                                on:change=move |ev| set_default_language.set(event_target_value(&ev))
                            >
                                <option value="en">"🇬🇧 English"</option>
                                <option value="fr">"🇫🇷 Français"</option>
                                <option value="it">"🇮🇹 Italiano"</option>
                            </select>
                        </div>

                        <div class="flex justify-end space-x-4 pt-6 border-t-2 border-gray-200 mt-6">
                            <button
                                type="button"
                                on:click=move |_| on_close.with_value(|f| f())
                                class="px-8 py-3 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-all font-bold shadow-lg hover:shadow-xl border-2 border-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                disabled=move || saving.get()
                            >
                                <span>"❌"</span>
                                <span>"Cancel"</span>
                            </button>
                            <button
                                type="submit"
                                on:click=handle_save
                                class="px-8 py-3 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-all font-bold shadow-lg hover:shadow-xl border-2 border-green-700 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:shadow-lg flex items-center gap-2"
                                disabled=move || saving.get() || name.get().is_empty()
                            >
                                {move || if saving.get() {
                                    view! {
                                        <>
                                            <span class="animate-spin">"⏳"</span>
                                            <span>"Saving..."</span>
                                        </>
                                    }.into_view()
                                } else if is_edit {
                                    view! {
                                        <>
                                            <span>"✅"</span>
                                            <span>"Update Guest Group"</span>
                                        </>
                                    }.into_view()
                                } else {
                                    view! {
                                        <>
                                            <span>"➕"</span>
                                            <span>"Add Guest Group"</span>
                                        </>
                                    }.into_view()
                                }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
