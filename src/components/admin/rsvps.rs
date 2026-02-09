use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{GuestGroup, RsvpWithDietaryCounts};
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn RsvpManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (rsvps, set_rsvps) = create_signal::<Vec<RsvpWithDietaryCounts>>(Vec::new());
    let (guests, set_guests) = create_signal::<HashMap<String, GuestGroup>>(HashMap::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (location_filter, set_location_filter) = create_signal::<Option<String>>(None);
    let (status_filter, set_status_filter) = create_signal::<Option<String>>(None);
    let (search_query, set_search_query) = create_signal(String::new());

    // Load RSVPs and guests
    let load_data = {
        create_action(move |_: &()| {
            async move {
                set_loading.set(true);
                set_error.set(None);

                // Load RSVPs with dietary counts
                let rsvps_result = admin_context
                    .authenticated_client()
                    .get_all_rsvps_with_dietary_counts()
                    .await;

                // Load guests
                let guests_result = admin_context
                    .authenticated_client()
                    .get_all_guest_groups()
                    .await;

                match (rsvps_result, guests_result) {
                    (Ok(rsvp_list), Ok(guest_list)) => {
                        set_rsvps.set(rsvp_list);

                        // Create a hashmap of guests for quick lookup
                        let guest_map: HashMap<String, GuestGroup> =
                            guest_list.into_iter().map(|g| (g.id.clone(), g)).collect();
                        set_guests.set(guest_map);

                        set_loading.set(false);
                    }
                    (Err(e), _) | (_, Err(e)) => {
                        set_error.set(Some(format!("Failed to load data: {}", e)));
                        set_loading.set(false);
                    }
                }
            }
        })
    };

    // Load data on mount
    create_effect(move |_| {
        load_data.dispatch(());
    });

    // Filter RSVPs based on criteria
    let filtered_rsvps = move || {
        let query = search_query.get().to_lowercase();
        let loc_filter = location_filter.get();
        let stat_filter = status_filter.get();
        let guest_map = guests.get();

        rsvps
            .get()
            .into_iter()
            .filter(|rsvp| {
                // Search filter (by guest name)
                let matches_search = if query.is_empty() {
                    true
                } else {
                    guest_map
                        .get(&rsvp.rsvp.guest_group_id)
                        .is_some_and(|g| g.name.to_lowercase().contains(&query))
                };

                // Location filter
                let matches_location = loc_filter
                    .as_ref()
                    .is_none_or(|loc| &rsvp.rsvp.location == loc);

                // Status filter
                let matches_status =
                    stat_filter
                        .as_ref()
                        .is_none_or(|status| match status.as_str() {
                            "attending" => rsvp.rsvp.attending,
                            "declined" => !rsvp.rsvp.attending,
                            _ => true,
                        });

                matches_search && matches_location && matches_status
            })
            .collect::<Vec<_>>()
    };

    // Calculate totals for filtered RSVPs
    let totals = move || {
        let filtered = filtered_rsvps();
        let attending = filtered.iter().filter(|r| r.rsvp.attending).count() as i32;
        let guests_count: i32 = filtered
            .iter()
            .filter(|r| r.rsvp.attending)
            .map(|r| r.rsvp.number_of_guests)
            .sum();
        let vegetarian: i32 = filtered
            .iter()
            .filter(|r| r.rsvp.attending)
            .map(|r| r.dietary_vegetarian)
            .sum();
        let vegan: i32 = filtered
            .iter()
            .filter(|r| r.rsvp.attending)
            .map(|r| r.dietary_vegan)
            .sum();
        let gluten_free: i32 = filtered
            .iter()
            .filter(|r| r.rsvp.attending)
            .map(|r| r.dietary_gluten_free)
            .sum();
        let other_dietary: i32 = filtered
            .iter()
            .filter(|r| r.rsvp.attending && !r.dietary_other.is_empty())
            .count() as i32;

        (
            attending,
            guests_count,
            vegetarian,
            vegan,
            gluten_free,
            other_dietary,
        )
    };

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "RSVP Management"
                </h2>
                <button
                    on:click=move |_| load_data.dispatch(())
                    class=REFRESH_BUTTON
                >
                    "↻ Refresh"
                </button>
            </div>

            {move || error.get().map(|err| view! {
                <div class=ALERT_ERROR>
                    {err}
                </div>
            })}

            {/* Summary Cards */}
            <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
                <SummaryCard
                    icon="✅"
                    label="Attending"
                    value=move || totals().0
                    color="green"
                />
                <SummaryCard
                    icon="👥"
                    label="Total Guests"
                    value=move || totals().1
                    color="blue"
                />
                <SummaryCard
                    icon="🥗"
                    label="Vegetarian"
                    value=move || totals().2
                    color="yellow"
                />
                <SummaryCard
                    icon="🌱"
                    label="Vegan"
                    value=move || totals().3
                    color="purple"
                />
                <SummaryCard
                    icon="🌾"
                    label="Gluten-Free"
                    value=move || totals().4
                    color="amber"
                />
                <SummaryCard
                    icon="🍽️"
                    label="Other Dietary"
                    value=move || totals().5
                    color="orange"
                />
            </div>

            {/* Search and Filters */}
            <div class=FILTER_SECTION>
                <div class=FILTER_GRID>
                    <div>
                        <label class=FORM_LABEL>
                            "Search by Guest Name"
                        </label>
                        <input
                            type="text"
                            placeholder="Search..."
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
                        </select>
                    </div>
                    <div>
                        <label class=FORM_LABEL>
                            "Filter by Status"
                        </label>
                        <select
                            class=FORM_SELECT
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_status_filter.set(if value.is_empty() { None } else { Some(value) });
                            }
                        >
                            <option value="">"All Statuses"</option>
                            <option value="attending">"Attending"</option>
                            <option value="declined">"Declined"</option>
                        </select>
                    </div>
                </div>
            </div>

            {/* RSVP Table */}
            {move || {
                if loading.get() {
                    view! {
                        <div class=LOADING_CONTAINER>
                            <div class=LOADING_SPINNER></div>
                        </div>
                    }.into_view()
                } else {
                    let rsvps_list = filtered_rsvps();
                    let guest_map = guests.get();

                    view! {
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                            <div class="overflow-x-auto">
                                <table class="min-w-full divide-y divide-gray-200">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Guest Name"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Location"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Status"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "# Guests"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Dietary"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Notes"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Date"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                "Actions"
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="bg-white divide-y divide-gray-200">
                                        {rsvps_list.into_iter().map(|rsvp_with_counts| {
                                            let guest_name = guest_map.get(&rsvp_with_counts.rsvp.guest_group_id)
                                                .map(|g| g.name.clone())
                                                .unwrap_or_else(|| "Unknown".to_string());

                                            let rsvp_id = rsvp_with_counts.rsvp.id.clone();
                                            let dietary_info = format_dietary(&rsvp_with_counts);
                                            let created_at = rsvp_with_counts.rsvp.created_at
                                                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                                .unwrap_or_else(|| "-".to_string());

                                            view! {
                                                <tr class="hover:bg-gray-50">
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {guest_name}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        <span class={format!("px-2 py-1 rounded-full text-xs font-semibold {}",
                                                            if rsvp_with_counts.rsvp.location == "sardinia" {
                                                                "bg-blue-100 text-blue-800"
                                                            } else {
                                                                "bg-green-100 text-green-800"
                                                            }
                                                        )}>
                                                            {rsvp_with_counts.rsvp.location.to_uppercase()}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        <span class={format!("px-2 py-1 rounded-full text-xs font-semibold {}",
                                                            if rsvp_with_counts.rsvp.attending {
                                                                "bg-green-100 text-green-800"
                                                            } else {
                                                                "bg-red-100 text-red-800"
                                                            }
                                                        )}>
                                                            {if rsvp_with_counts.rsvp.attending { "Attending" } else { "Declined" }}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {if rsvp_with_counts.rsvp.attending { rsvp_with_counts.rsvp.number_of_guests.to_string() } else { "-".to_string() }}
                                                    </td>
                                                    <td class="px-6 py-4 text-sm text-gray-500">
                                                        <div class="max-w-xs">
                                                            {dietary_info}
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-4 text-sm text-gray-500">
                                                        <div class="max-w-xs truncate" title={rsvp_with_counts.rsvp.additional_notes.clone().unwrap_or_default()}>
                                                            {rsvp_with_counts.rsvp.additional_notes.clone().unwrap_or_else(|| "-".to_string())}
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {created_at}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                                                        <button
                                                            on:click={
                                                                move |_| {
                                                                    let confirmed = window().confirm_with_message("Are you sure you want to delete this RSVP?").unwrap_or(false);
                                                                    if confirmed {
                                                                        let rsvp_id = rsvp_id.clone();
                                                                        spawn_local(async move {
                                                                            match admin_context.authenticated_client().delete_rsvp(&rsvp_id).await {
                                                                                Ok(_) => {
                                                                                    load_data.dispatch(());
                                                                                }
                                                                                Err(e) => {
                                                                                    leptos::logging::error!("Failed to delete RSVP: {}", e);
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            class="px-2 py-1 bg-red-600 text-white rounded hover:bg-red-700 text-xs"
                                                        >
                                                            "Delete"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                            <div class="px-6 py-4 bg-gray-50 border-t border-gray-200">
                                <p class="text-sm text-gray-700">
                                    "Showing " <span class="font-medium">{move || filtered_rsvps().len()}</span>
                                    " of " <span class="font-medium">{move || rsvps.get().len()}</span> " RSVPs"
                                </p>
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn SummaryCard(
    icon: &'static str,
    label: &'static str,
    value: impl Fn() -> i32 + 'static,
    color: &'static str,
) -> impl IntoView {
    let bg_color = match color {
        "green" => "bg-green-50",
        "blue" => "bg-blue-50",
        "yellow" => "bg-yellow-50",
        "purple" => "bg-purple-50",
        "orange" => "bg-orange-50",
        _ => "bg-gray-50",
    };

    let text_color = match color {
        "green" => "text-green-600",
        "blue" => "text-blue-600",
        "yellow" => "text-yellow-600",
        "purple" => "text-purple-600",
        "orange" => "text-orange-600",
        _ => "text-gray-600",
    };

    view! {
        <div class={format!("rounded-lg shadow-md p-6 {}", bg_color)}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-gray-600 mb-1">{label}</p>
                    <p class={format!("text-3xl font-bold {}", text_color)}>{move || value()}</p>
                </div>
                <div class="text-4xl">{icon}</div>
            </div>
        </div>
    }
}

fn format_dietary(rsvp: &RsvpWithDietaryCounts) -> String {
    if !rsvp.rsvp.attending {
        return "-".to_string();
    }

    let mut parts = Vec::new();

    if rsvp.dietary_vegetarian > 0 {
        parts.push(format!("🥗 {} vegetarian", rsvp.dietary_vegetarian));
    }

    if rsvp.dietary_vegan > 0 {
        parts.push(format!("🌱 {} vegan", rsvp.dietary_vegan));
    }

    if rsvp.dietary_gluten_free > 0 {
        parts.push(format!("🌾 {} gluten-free", rsvp.dietary_gluten_free));
    }

    if !rsvp.dietary_other.is_empty() {
        parts.push(format!("📝 {}", rsvp.dietary_other.join(", ")));
    }

    if parts.is_empty() {
        "None".to_string()
    } else {
        parts.join(", ")
    }
}
