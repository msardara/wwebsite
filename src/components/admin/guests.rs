use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{
    DietaryPreferences, Guest, GuestGroup, GuestGroupInput, GuestGroupUpdate, GuestGroupWithCount,
    GuestInput,
};
use leptos::*;
use web_sys::console;

#[component]
pub fn GuestManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (guests, set_guests) = create_signal::<Vec<GuestGroupWithCount>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (show_add_modal, set_show_add_modal) = create_signal(false);
    let (edit_guest, set_edit_guest) = create_signal::<Option<GuestGroup>>(None);
    let (search_query, set_search_query) = create_signal(String::new());
    let (location_filter, set_location_filter) = create_signal::<Option<String>>(None);
    let (expanded_guest, set_expanded_guest) = create_signal::<Option<String>>(None);
    let (guest_invitees, set_guest_invitees) = create_signal::<Vec<Guest>>(Vec::new());
    let (loading_invitees, set_loading_invitees) = create_signal(false);

    // Load guests
    let load_guests = {
        create_action(move |_: &()| async move {
            set_loading.set(true);
            set_error.set(None);

            match admin_context
                .authenticated_client()
                .get_all_guest_groups_with_count()
                .await
            {
                Ok(guest_groups) => {
                    set_guests.set(guest_groups);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load guests: {}", e)));
                    set_loading.set(false);
                }
            }
        })
    };

    // Load guests on mount
    create_effect(move |_| {
        load_guests.dispatch(());
    });

    // Filter guests based on search and location
    let filtered_guests = move || {
        let query = search_query.get().to_lowercase();
        let loc_filter = location_filter.get();

        guests
            .get()
            .into_iter()
            .filter(|guest_with_count| {
                let guest = &guest_with_count.guest_group;
                let matches_search = query.is_empty()
                    || guest.name.to_lowercase().contains(&query)
                    || guest
                        .email
                        .as_ref()
                        .is_some_and(|e| e.to_lowercase().contains(&query))
                    || guest.invitation_code.to_lowercase().contains(&query);

                let matches_location = loc_filter
                    .as_ref()
                    .is_none_or(|loc| guest.locations.join(", ") == *loc);

                matches_search && matches_location
            })
            .collect::<Vec<_>>()
    };

    // Load invitees for a guest
    let load_invitees = {
        move |guest_id: String| {
            spawn_local(async move {
                set_loading_invitees.set(true);
                match admin_context
                    .authenticated_client()
                    .get_guests_admin(&guest_id)
                    .await
                {
                    Ok(invitees_list) => {
                        set_guest_invitees.set(invitees_list);
                        set_loading_invitees.set(false);
                    }
                    Err(_) => {
                        set_guest_invitees.set(Vec::new());
                        set_loading_invitees.set(false);
                    }
                }
            });
        }
    };

    // Toggle guest expansion
    let toggle_guest = store_value(move |guest_id: String| {
        if expanded_guest.get().as_ref() == Some(&guest_id) {
            set_expanded_guest.set(None);
            set_guest_invitees.set(Vec::new());
        } else {
            set_expanded_guest.set(Some(guest_id.clone()));
            load_invitees(guest_id);
        }
    });

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "Guest Management"
                </h2>
                <button
                    on:click=move |_| set_show_add_modal.set(true)
                    class=BUTTON_SECONDARY_INLINE
                >
                    <span>"+ Add Guest Group"</span>
                </button>
            </div>

            {move || error.get().map(|err| view! {
                <div class=ALERT_ERROR>
                    {err}
                </div>
            })}

            {/* Search and Filter */}
            <div class=FILTER_SECTION>
                <div class=GRID_2_COLS>
                    <div>
                        <label class=FORM_LABEL>
                            "Search"
                        </label>
                        <input
                            type="text"
                            placeholder="Search by name, email, or code..."
                            class=FORM_INPUT
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            prop:value=move || search_query.get()
                        />
                    </div>
                    <div>
                        <label class=FORM_LABEL>
                            "Filter by Location"
                        </label>
                        <select
                            class=FORM_SELECT
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_location_filter.set(if value.is_empty() { None } else { Some(value) });
                            }
                        >
                            <option value="">"All Locations"</option>
                            <option value="sardinia">"Sardinia"</option>
                            <option value="tunisia">"Tunisia"</option>
                            <option value="both">"Both"</option>
                        </select>
                    </div>
                </div>
            </div>

            {/* Guest Cards */}
            {move || {
                if loading.get() {
                    view! {
                        <div class=LOADING_CONTAINER>
                            <div class="text-center">
                                <div class=LOADING_SPINNER></div>
                                <p class="text-gray-600 font-medium mt-4">"Loading guests..."</p>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    let guests_list = filtered_guests();

                    if guests_list.is_empty() {
                        view! {
                            <div class=EMPTY_STATE>
                                <div class=EMPTY_STATE_ICON>"👥"</div>
                                <h3 class=EMPTY_STATE_TITLE>"No guests found"</h3>
                                <p class=EMPTY_STATE_MESSAGE>"Add your first guest group to get started"</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="space-y-4">
                                {guests_list.into_iter().map(|guest_with_count| {
                                    let guest = guest_with_count.guest_group.clone();
                                    let guest_count = guest_with_count.guest_count;
                                    let guest_id = guest.id.clone();
                                    let guest_id_for_expand = guest.id.clone();
                                    let guest_id_for_expand2 = guest.id.clone();
                                    let guest_id_for_delete = guest.id.clone();
                                    let guest_for_edit = guest.clone();

                                    view! {
                                        <div class="bg-white rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200 overflow-hidden">
                                            {/* Card Content */}
                                            <div class="p-6">
                                                <div class="flex items-start gap-8">
                                                    {/* Expand Button */}
                                                    <button
                                                        on:click={
                                                            let guest_id = guest_id.clone();
                                                            move |_| toggle_guest.with_value(|f| f(guest_id.clone()))
                                                        }
                                                        class="flex-shrink-0 w-8 h-8 flex items-center justify-center rounded-xl bg-gradient-to-br from-secondary-500 to-secondary-700 hover:from-secondary-600 hover:to-secondary-800 transition-all duration-200 font-bold text-lg shadow-lg hover:shadow-xl"
                                                        title="Click to view guest list"
                                                    >
                                                        {move || if expanded_guest.get().as_ref() == Some(&guest_id_for_expand) { "▼" } else { "▶" }}
                                                    </button>

                                                    {/* Main Content */}
                                                    <div class="flex-1">
                                                        {/* Top Row - Name and Actions */}
                                                        <div class="flex items-start justify-between gap-4 mb-6">
                                                            {/* Name & Email */}
                                                            <div>
                                                                <h3 class="text-2xl font-bold text-gray-900">{guest.name.clone()}</h3>
                                                                <p class="text-lg font-semibold text-gray-700">
                                                                    {guest.email.clone().unwrap_or_else(|| "No email".to_string())}
                                                                </p>
                                                            </div>

                                                            {/* Actions */}
                                                            <div class="flex items-center gap-3">
                                                                <button
                                                                    on:click=move |_| set_edit_guest.set(Some(guest_for_edit.clone()))
                                                                    class=BUTTON_PRIMARY_INLINE
                                                                >
                                                                    "✏️ Edit"
                                                                </button>
                                                                <button
                                                                    on:click={
                                                                        move |_| {
                                                                            let confirmed = window().confirm_with_message("Are you sure you want to delete this guest group?").unwrap_or(false);
                                                                            if confirmed {
                                                                                let guest_id = guest_id_for_delete.clone();
                                                                                spawn_local(async move {
                                                                                    match admin_context.authenticated_client().delete_guest_group(&guest_id).await {
                                                                                        Ok(_) => {
                                                                                            load_guests.dispatch(());
                                                                                        }
                                                                                        Err(e) => {
                                                                                            set_error.set(Some(format!("Failed to delete guest group: {}", e)));
                                                                                        }
                                                                                    }
                                                                                });
                                                                            }
                                                                        }
                                                                    }
                                                                    class=BUTTON_DANGER_INLINE
                                                                >
                                                                    "🗑️"
                                                                </button>
                                                            </div>
                                                        </div>

                                                        {/* Bottom Row - Details Flex */}
                                                        <div class="flex items-start gap-8">
                                                            {/* Invitation Code */}
                                                            <div class="flex-1">
                                                                <div class="text-sm font-extrabold text-gray-900 uppercase mb-2">"INVITATION CODE"</div>
                                                                {
                                                                    let (copied, set_copied) = create_signal(false);
                                                                    let code_for_display = guest.invitation_code.clone();

                                                                    view! {
                                                                        <button
                                                                            on:click={
                                                                                let code = guest.invitation_code.clone();
                                                                                move |_| {
                                                                                    let win = window();
                                                                                    if let Ok(origin) = win.location().origin() {
                                                                                        let invitation_url = format!("{}/invitation?code={}", origin, code);

                                                                                        // Use web-sys Clipboard API
                                                                                        let clipboard = win.navigator().clipboard();
                                                                                        let _ = clipboard.write_text(&invitation_url);

                                                                                        // Show visual feedback
                                                                                        set_copied.set(true);
                                                                                        set_timeout(move || {
                                                                                            set_copied.set(false);
                                                                                        }, std::time::Duration::from_secs(2));
                                                                                    }
                                                                                }
                                                                            }
                                                                            class=move || {
                                                                                if copied.get() {
                                                                                    "px-5 py-3 bg-green-100 text-green-900 text-xl font-mono font-bold rounded-lg shadow-lg border-2 border-green-400 transition-all cursor-pointer inline-block w-[220px] text-center"
                                                                                } else {
                                                                                    "px-5 py-3 bg-gray-100 text-gray-900 text-xl font-mono font-bold rounded-lg shadow-lg border-2 border-gray-300 hover:bg-primary-100 hover:border-primary-400 transition-all cursor-pointer inline-block w-[220px] text-center"
                                                                                }
                                                                            }
                                                                            title="Click to copy invitation link"
                                                                        >
                                                                            {move || if copied.get() {
                                                                                "✓Copied!".to_string()
                                                                            } else {
                                                                                code_for_display.clone()
                                                                            }}
                                                                        </button>
                                                                    }
                                                                }
                                                            </div>

                                                            {/* Party Size */}
                                                            <div class="flex-1">
                                                                <div class="text-sm font-extrabold text-gray-900 uppercase mb-2">"PARTY SIZE"</div>
                                                                <div class="px-5 py-3 bg-gray-100 text-gray-900 text-xl font-mono font-bold rounded-lg shadow-lg inline-block border-2 border-gray-300">
                                                                    {guest_count}
                                                                </div>
                                                            </div>

                                                            {/* Location */}
                                                            <div class="flex-1">
                                                                <div class="text-sm font-extrabold text-gray-900 uppercase mb-2">"LOCATION"</div>
                                                                <div class="px-5 py-3 bg-gray-100 text-gray-900 text-xl font-mono font-bold rounded-lg shadow-lg inline-block border-2 border-gray-300">
                                                                    {guest.locations.join(", ")}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>

                                            {/* Expanded Guest List */}
                                            {move || if expanded_guest.get().as_ref() == Some(&guest_id_for_expand2) {
                                                let guest_notes = guest.additional_notes.clone();
                                                view! {
                                                    <div class="bg-gradient-to-br from-secondary-50 to-secondary-100 border-t-2 border-secondary-200 p-6">
                                                        {/* Additional Notes Section */}
                                                        {guest_notes.as_ref().and_then(|notes| {
                                                            if !notes.trim().is_empty() {
                                                                Some(view! {
                                                                    <div class="bg-amber-50 rounded-xl shadow-lg p-5 mb-6 border-2 border-amber-200">
                                                                        <div class="flex items-start gap-3">
                                                                            <span class="flex-shrink-0 text-2xl">"📝"</span>
                                                                            <div class="flex-1">
                                                                                <h5 class="text-sm font-bold text-amber-900 uppercase tracking-wide mb-2">"Additional Notes"</h5>
                                                                                <p class="text-sm text-amber-900 whitespace-pre-wrap leading-relaxed">{notes.clone()}</p>
                                                                            </div>
                                                                        </div>
                                                                    </div>
                                                                })
                                                            } else {
                                                                None
                                                            }
                                                        })}

                                                        <div class="bg-white rounded-xl shadow-lg p-6 border-2 border-secondary-200">
                                                            <h4 class="text-lg font-bold text-secondary-800 mb-5 flex items-center gap-3 pb-4 border-b-2 border-secondary-200">
                                                                <span class="text-2xl">"👥"</span>
                                                                <span>"Guest List"</span>
                                                            </h4>
                                                            {move || if loading_invitees.get() {
                                                                view! {
                                                                    <div class="flex items-center justify-center gap-3 py-8 text-gray-500">
                                                                        <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-secondary-600"></div>
                                                                        <span class="text-sm font-medium">"Loading guests..."</span>
                                                                    </div>
                                                                }.into_view()
                                                            } else {
                                                                let invitees_list = guest_invitees.get();
                                                                if invitees_list.is_empty() {
                                                                    view! {
                                                                        <div class="text-center py-8">
                                                                            <p class="text-gray-500 text-sm italic">"No guests have been added to this group yet"</p>
                                                                        </div>
                                                                    }.into_view()
                                                                } else {
                                                                    view! {
                                                                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                                                            {invitees_list.into_iter().map(|invitee| {
                                                                                let has_dietary = invitee.dietary_preferences.vegetarian
                                                                                    || invitee.dietary_preferences.vegan
                                                                                    || invitee.dietary_preferences.halal
                                                                                    || invitee.dietary_preferences.no_pork
                                                                                    || invitee.dietary_preferences.gluten_free
                                                                                    || !invitee.dietary_preferences.other.is_empty();

                                                                                let attending_locs = invitee.attending_locations.clone();
                                                                                let has_attending = !attending_locs.is_empty();
                                                                                let age_display = invitee.age_category.display_name().to_string();

                                                                                view! {
                                                                                    <div class="bg-white rounded-lg border-2 border-secondary-200 p-4 hover:shadow-lg transition-all duration-200">
                                                                                        <div class="flex flex-col gap-3">
                                                                                            {/* Guest Name */}
                                                                                            <div class="flex items-center gap-2">
                                                                                                <div class="flex-shrink-0 w-8 h-8 bg-gradient-to-br from-secondary-500 to-secondary-700 text-white rounded-full flex items-center justify-center font-bold text-sm shadow-md">
                                                                                                    {invitee.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                                                                                </div>
                                                                                                <div class="flex-1">
                                                                                                    <p class="text-base font-bold text-gray-900">{invitee.name}</p>
                                                                                                    <p class="text-xs text-gray-500">
                                                                                                        {age_display}
                                                                                                    </p>
                                                                                                </div>
                                                                                            </div>

                                                                                            {/* Attending Locations */}
                                                                                            <div class="border-t border-gray-100 pt-2">
                                                                                                <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1.5">"Attending"</p>
                                                                                                {if has_attending {
                                                                                                    view! {
                                                                                                        <div class="flex flex-wrap gap-1.5">
                                                                                                            {attending_locs.iter().map(|loc| {
                                                                                                                let (icon, color) = match loc.as_str() {
                                                                                                                    "sardinia" => ("🇮🇹", "bg-blue-100 text-blue-800 border-blue-300"),
                                                                                                                    "tunisia" => ("🇹🇳", "bg-red-100 text-red-800 border-red-300"),
                                                                                                                    _ => ("📍", "bg-gray-100 text-gray-800 border-gray-300"),
                                                                                                                };
                                                                                                                view! {
                                                                                                                    <span class={format!("px-2 py-1 text-xs font-semibold rounded-full border {}", color)}>
                                                                                                                        {format!("{} {}", icon, loc)}
                                                                                                                    </span>
                                                                                                                }
                                                                                                            }).collect::<Vec<_>>()}
                                                                                                        </div>
                                                                                                    }.into_view()
                                                                                                } else {
                                                                                                    view! {
                                                                                                        <p class="text-xs text-gray-400 italic">"Not confirmed yet"</p>
                                                                                                    }.into_view()
                                                                                                }}
                                                                                            </div>

                                                                                            {/* Dietary Preferences */}
                                                                                            <div class="border-t border-gray-100 pt-2">
                                                                                                <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1.5">"Dietary Preferences"</p>
                                                                                                {if has_dietary {
                                                                                                    view! {
                                                                                                        <div class="flex flex-wrap gap-1.5">
                                                                                                            {invitee.dietary_preferences.vegetarian.then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded-full border border-green-300">"🌱 Vegetarian"</span>
                                                                                                            })}
                                                                                                            {invitee.dietary_preferences.vegan.then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded-full border border-green-300">"🥬 Vegan"</span>
                                                                                                            })}
                                                                                                            {invitee.dietary_preferences.halal.then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded-full border border-purple-300">"☪️ Halal"</span>
                                                                                                            })}
                                                                                                            {invitee.dietary_preferences.no_pork.then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-pink-100 text-pink-800 text-xs font-semibold rounded-full border border-pink-300">"🚫🐷 No Pork"</span>
                                                                                                            })}
                                                                                                            {invitee.dietary_preferences.gluten_free.then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-yellow-100 text-yellow-800 text-xs font-semibold rounded-full border border-yellow-300">"🌾 Gluten-Free"</span>
                                                                                                            })}
                                                                                                            {(!invitee.dietary_preferences.other.is_empty()).then(|| view! {
                                                                                                                <span class="px-2 py-1 bg-orange-100 text-orange-800 text-xs font-semibold rounded-full border border-orange-300">{format!("ℹ️ {}", invitee.dietary_preferences.other.clone())}</span>
                                                                                                            })}
                                                                                                        </div>
                                                                                                    }.into_view()
                                                                                                } else {
                                                                                                    view! {
                                                                                                        <p class="text-xs text-gray-400 italic">"None specified"</p>
                                                                                                    }.into_view()
                                                                                                }}
                                                                                            </div>
                                                                                        </div>
                                                                                    </div>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </div>
                                                                    }.into_view()
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                ().into_view()
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}

                                {/* Summary Card */}
                                <div class="bg-gradient-to-r from-gray-50 to-gray-100 rounded-xl shadow-md p-6 border border-gray-200">
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center gap-6">
                                            <div class="bg-white rounded-lg px-6 py-4 shadow-sm border border-gray-200">
                                                <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">"Guest Groups"</p>
                                                <p class="text-3xl font-bold text-gray-900">{move || filtered_guests().len()}</p>
                                            </div>
                                            <div class="bg-white rounded-lg px-6 py-4 shadow-sm border border-gray-200">
                                                <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">"Total Guests"</p>
                                                <p class="text-3xl font-bold text-primary-600">
                                                    {move || {
                                                        let total: i32 = filtered_guests().iter().map(|g| g.guest_count).sum();
                                                        total
                                                    }}
                                                </p>
                                            </div>
                                        </div>
                                        {move || {
                                            let total_groups = guests.get().len();
                                            let filtered_count = filtered_guests().len();
                                            if filtered_count < total_groups {
                                                view! {
                                                    <p class="text-sm text-gray-600 font-medium">
                                                        {format!("Showing {} of {} groups", filtered_count, total_groups)}
                                                    </p>
                                                }.into_view()
                                            } else {
                                                ().into_view()
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    }
                }
            }}

            {/* Add Guest Modal */}
            {
                move || show_add_modal.get().then(|| {
                    view! {
                        <GuestgroupModal
                            guest=None
                            on_close=move || set_show_add_modal.set(false)
                            on_save=move || {
                                set_show_add_modal.set(false);
                                load_guests.dispatch(());
                            }
                        />
                    }
                })
            }

            {/* Edit Guest Modal */}
            {
                move || edit_guest.get().map(|guest| {
                    view! {
                        <GuestgroupModal
                            guest=Some(guest)
                            on_close=move || set_edit_guest.set(None)
                            on_save=move || {
                                set_edit_guest.set(None);
                                load_guests.dispatch(());
                            }
                        />
                    }
                })
            }
        </div>
    }
}

#[derive(Debug, Clone)]
struct GuestRow {
    id: usize,
    first_name: String,
    last_name: String,
    age_category: crate::types::AgeCategory,
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
fn GuestgroupModal(
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
    let (invitation_code, set_invitation_code) = create_signal(
        guest
            .as_ref()
            .map_or_else(generate_invitation_code, |g| g.invitation_code.clone()),
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
        if locs.is_empty() {
            locs.push("sardinia".to_string()); // Default
        }
        locs
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
                        invitation_code: invitation_code.get(),
                        party_size: actual_party_size,
                        locations: get_locations_value(),
                        default_language: default_language.get(),
                        additional_notes: None,
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

                        {if !is_edit {
                            view! {
                                <div class="bg-gray-50 rounded-lg p-4 border border-gray-200">
                                    <label class="block text-sm font-semibold text-gray-700 mb-2">
                                        "Invitation Code" <span class="text-red-500">"*"</span>
                                    </label>
                                    <div class="flex gap-2">
                                        <input
                                            type="text"
                                            required
                                            placeholder="XXXXXXXX"
                                            class="flex-1 px-4 py-3 border-2 border-gray-300 rounded-lg focus:ring-2 focus:ring-secondary-500 focus:border-secondary-500 font-mono text-lg font-bold tracking-wider"
                                            prop:value=move || invitation_code.get()
                                            on:input=move |ev| set_invitation_code.set(event_target_value(&ev))
                                        />
                                        <button
                                            type="button"
                                            on:click=move |_| set_invitation_code.set(generate_invitation_code())
                                            class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-all font-semibold shadow-md hover:shadow-lg flex items-center gap-2"
                                        >
                                            <span>"🔄"</span>
                                            <span>"Generate"</span>
                                        </button>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}

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

fn generate_invitation_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
