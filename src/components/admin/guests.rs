use super::guest_group_modal::GuestgroupModal;
use crate::components::common::LocationBadge;
use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Guest, GuestGroup, GuestGroupWithCount, Location};
use leptos::*;

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

    // Load guests on mount (fire once, not inside a reactive effect)
    request_animation_frame(move || {
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
                                                                                let has_dietary = invitee.dietary_preferences.has_any();
                                                                                let dietary_badges = invitee.dietary_preferences.as_badges();
                                                                                let other_badge = invitee.dietary_preferences.other_badge();

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
                                                                                                            {attending_locs.iter().filter_map(|loc| {
                                                                                                                Location::from_str(loc).map(|l| view! { <LocationBadge location=l /> })
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
                                                                                                            {dietary_badges.iter().map(|(label, css)| {
                                                                                                                view! {
                                                                                                                    <span class={format!("px-2 py-1 text-xs font-semibold rounded-full border {}", css)}>{*label}</span>
                                                                                                                }
                                                                                                            }).collect::<Vec<_>>()}
                                                                                                            {other_badge.map(|(label, css)| view! {
                                                                                                                <span class={format!("px-2 py-1 text-xs font-semibold rounded-full border {}", css)}>{label}</span>
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
