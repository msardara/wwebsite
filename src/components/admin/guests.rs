use super::guest_group_modal::GuestgroupModal;
use crate::components::common::LocationBadge;
use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Guest, GuestGroup, GuestGroupUpdate, GuestGroupWithCount, Location};
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
    let (loc_sardinia, set_loc_sardinia) = create_signal(false);
    let (loc_tunisia, set_loc_tunisia) = create_signal(false);
    let (loc_nice, set_loc_nice) = create_signal(false);
    let (invitation_filter, set_invitation_filter) = create_signal::<Option<bool>>(None);
    let (invited_by_filter, set_invited_by_filter) = create_signal::<Option<String>>(None);
    let (expanded_guest, set_expanded_guest) = create_signal::<Option<String>>(None);
    let (guest_invitees, set_guest_invitees) = create_signal::<Vec<Guest>>(Vec::new());

    // Sub-tab: "groups" | "all"
    let (active_tab, set_active_tab) = create_signal("groups");

    // All-guests flat list
    let (all_guests, set_all_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (all_guests_loading, set_all_guests_loading) = create_signal(false);
    let (all_guests_search, set_all_guests_search) = create_signal(String::new());

    // Filters for All Guests tab (independent from Guest Groups tab)
    let (ag_loc_sardinia, set_ag_loc_sardinia) = create_signal(false);
    let (ag_loc_tunisia, set_ag_loc_tunisia) = create_signal(false);
    let (ag_loc_nice, set_ag_loc_nice) = create_signal(false);
    let (ag_invitation_filter, set_ag_invitation_filter) = create_signal::<Option<bool>>(None);
    let (ag_invited_by_filter, set_ag_invited_by_filter) = create_signal::<Option<String>>(None);

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

    let load_all_guests = create_action(move |_: &()| async move {
        set_all_guests_loading.set(true);
        match admin_context.authenticated_client().get_all_guests().await {
            Ok(list) => {
                set_all_guests.set(list);
                set_all_guests_loading.set(false);
            }
            Err(e) => {
                set_error.set(Some(format!("Failed to load all guests: {}", e)));
                set_all_guests_loading.set(false);
            }
        }
    });

    // Build lookups from the groups list for the All Guests tab
    let group_name_lookup = move || {
        guests
            .get()
            .into_iter()
            .map(|g| (g.guest_group.id.clone(), g.guest_group.name.clone()))
            .collect::<std::collections::HashMap<_, _>>()
    };
    let group_invited_by_lookup = move || {
        guests
            .get()
            .into_iter()
            .map(|g| (g.guest_group.id.clone(), g.guest_group.invited_by.clone()))
            .collect::<std::collections::HashMap<_, _>>()
    };
    let group_locations_lookup = move || {
        guests
            .get()
            .into_iter()
            .map(|g| (g.guest_group.id.clone(), g.guest_group.locations.clone()))
            .collect::<std::collections::HashMap<_, _>>()
    };
    let group_invitation_sent_lookup = move || {
        guests
            .get()
            .into_iter()
            .map(|g| (g.guest_group.id.clone(), g.guest_group.invitation_sent))
            .collect::<std::collections::HashMap<_, _>>()
    };

    // Sort state for All Guests tab
    let (sort_col, set_sort_col) = create_signal("name");
    let (sort_asc, set_sort_asc) = create_signal(true);

    // Load guests on mount (fire once, not inside a reactive effect)
    request_animation_frame(move || {
        load_guests.dispatch(());
        load_all_guests.dispatch(());
    });

    // Filter guests based on search and location
    let filtered_guests = move || {
        let query = search_query.get().to_lowercase();
        let filter_sardinia = loc_sardinia.get();
        let filter_tunisia = loc_tunisia.get();
        let filter_nice = loc_nice.get();
        let inv_filter = invitation_filter.get();
        let invited_filter = invited_by_filter.get();

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

                // If no checkbox is ticked, show all; otherwise show groups that
                // include at least one of the selected locations.
                let any_loc_checked = filter_sardinia || filter_tunisia || filter_nice;
                let matches_location = !any_loc_checked
                    || (filter_sardinia && guest.locations.contains(&"sardinia".to_string()))
                    || (filter_tunisia && guest.locations.contains(&"tunisia".to_string()))
                    || (filter_nice && guest.locations.contains(&"nice".to_string()));

                let matches_invitation =
                    inv_filter.is_none_or(|sent| guest.invitation_sent == sent);

                let matches_invited_by = invited_filter
                    .as_ref()
                    .is_none_or(|email| guest.invited_by.contains(email));

                matches_search && matches_location && matches_invitation && matches_invited_by
            })
            .collect::<Vec<_>>()
    };

    // Toggle guest expansion — reuses the already-loaded all_guests signal (no API call)
    let toggle_guest = store_value(move |guest_id: String| {
        if expanded_guest.get().as_ref() == Some(&guest_id) {
            set_expanded_guest.set(None);
            set_guest_invitees.set(Vec::new());
        } else {
            let invitees: Vec<_> = all_guests
                .get()
                .into_iter()
                .filter(|g| g.guest_group_id == guest_id)
                .collect();
            set_guest_invitees.set(invitees);
            set_expanded_guest.set(Some(guest_id));
        }
    });

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "Guest Management"
                </h2>
                {move || if active_tab.get() == "groups" {
                    view! {
                        <button
                            on:click=move |_| set_show_add_modal.set(true)
                            class=BUTTON_SECONDARY_INLINE
                        >
                            <span>"+ Add Guest Group"</span>
                        </button>
                    }.into_view()
                } else {
                    ().into_view()
                }}
            </div>

            {/* Sub-tabs */}
            <div class="flex gap-1 border-b border-gray-200 mb-4">
                <button
                    on:click=move |_| set_active_tab.set("groups")
                    class=move || if active_tab.get() == "groups" {
                        "px-4 py-2 text-sm font-semibold border-b-2 border-secondary-600 text-secondary-700 -mb-px"
                    } else {
                        "px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 border-b-2 border-transparent -mb-px"
                    }
                >
                    "👥 Guest Groups"
                </button>
                <button
                    on:click=move |_| set_active_tab.set("all")
                    class=move || if active_tab.get() == "all" {
                        "px-4 py-2 text-sm font-semibold border-b-2 border-secondary-600 text-secondary-700 -mb-px"
                    } else {
                        "px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 border-b-2 border-transparent -mb-px"
                    }
                >
                    "🧍 All Guests"
                </button>
            </div>

            {move || error.get().map(|err| view! {
                <div class=ALERT_ERROR>
                    {err}
                </div>
            })}

            {/* ── Guest Groups tab ── */}
            <Show when=move || active_tab.get() == "groups" fallback=|| ()>

            {/* Search and Filter */}
            <div class=FILTER_SECTION>
                <div class=GRID_4_COLS>
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
                        <div class="flex flex-col gap-1 mt-1">
                            <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                <input
                                    type="checkbox"
                                    class="w-4 h-4 text-secondary-600 rounded"
                                    prop:checked=move || loc_sardinia.get()
                                    on:change=move |ev| set_loc_sardinia.set(event_target_checked(&ev))
                                />
                                "🇮🇹 Sardinia"
                            </label>
                            <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                <input
                                    type="checkbox"
                                    class="w-4 h-4 text-secondary-600 rounded"
                                    prop:checked=move || loc_tunisia.get()
                                    on:change=move |ev| set_loc_tunisia.set(event_target_checked(&ev))
                                />
                                "🇹🇳 Tunisia"
                            </label>
                            <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                <input
                                    type="checkbox"
                                    class="w-4 h-4 text-secondary-600 rounded"
                                    prop:checked=move || loc_nice.get()
                                    on:change=move |ev| set_loc_nice.set(event_target_checked(&ev))
                                />
                                "🇫🇷 Nice"
                            </label>
                        </div>
                    </div>
                    <div>
                        <label class=FORM_LABEL>
                            "Filter by Invitation"
                        </label>
                        <select
                            class=FORM_SELECT
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_invitation_filter.set(match value.as_str() {
                                    "sent" => Some(true),
                                    "not_sent" => Some(false),
                                    _ => None,
                                });
                            }
                        >
                            <option value="">"All"</option>
                            <option value="sent">"Sent"</option>
                            <option value="not_sent">"Not Sent"</option>
                        </select>
                    </div>
                    <div>
                        <label class=FORM_LABEL>
                            "Filter by Invited By"
                        </label>
                        <select
                            class=FORM_SELECT
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_invited_by_filter.set(if value.is_empty() { None } else { Some(value) });
                            }
                        >
                            <option value="">"All"</option>
                            <option value="mauro.sardara@gmail.com">"Mauro"</option>
                            <option value="munaamamu0@gmail.com">"Muna"</option>
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
                                    let guest_id_for_invite = guest.id.clone();
                                    let (invitation_sent, set_invitation_sent) = create_signal(guest.invitation_sent);

                                    view! {
                                        <div class="bg-white rounded-lg shadow hover:shadow-md transition-all duration-200 border border-gray-200 overflow-hidden">
                                            {/* Card Content */}
                                            <div class="px-3 py-3 space-y-3">
                                                {/* Row 1: expand + name/email + edit/delete buttons */}
                                                <div class="flex items-center gap-2">
                                                    {/* Expand Button */}
                                                    <button
                                                        on:click={
                                                            let guest_id = guest_id.clone();
                                                            move |_| toggle_guest.with_value(|f| f(guest_id.clone()))
                                                        }
                                                        class="flex-shrink-0 w-6 h-6 flex items-center justify-center rounded bg-secondary-100 hover:bg-secondary-200 text-secondary-700 transition-all text-xs font-bold"
                                                        title="Click to view guest list"
                                                    >
                                                        {move || if expanded_guest.get().as_ref() == Some(&guest_id_for_expand) { "▼" } else { "▶" }}
                                                    </button>

                                                    {/* Name & Email */}
                                                    <div class="flex-1 min-w-0">
                                                        <h3 class="text-sm font-bold text-gray-900 truncate">{guest.name.clone()}</h3>
                                                        <p class="text-xs text-gray-500 truncate">
                                                            {guest.email.clone().unwrap_or_else(|| "No email".to_string())}
                                                        </p>
                                                    </div>

                                                    {/* Edit + Delete buttons */}
                                                    <div class="flex items-center gap-1.5 flex-shrink-0">
                                                        <button
                                                            on:click=move |_| set_edit_guest.set(Some(guest_for_edit.clone()))
                                                            class="px-2 py-1 text-xs font-semibold rounded border bg-primary-50 text-primary-700 border-primary-200 hover:bg-primary-100 transition-all"
                                                        >
                                                            "✏️"
                                                        </button>
                                                        <button
                                                            on:click={
                                                                move |_| {
                                                                    let confirmed = window().confirm_with_message("Are you sure you want to delete this guest group?").unwrap_or(false);
                                                                    if confirmed {
                                                                        let guest_id = guest_id_for_delete.clone();
                                                                        spawn_local(async move {
                                                                            match admin_context.authenticated_client().delete_guest_group(&guest_id).await {
                                                                                Ok(_) => load_guests.dispatch(()),
                                                                                Err(e) => set_error.set(Some(format!("Failed to delete guest group: {}", e))),
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            class="px-2 py-1 text-xs font-semibold rounded border bg-red-50 text-red-600 border-red-200 hover:bg-red-100 transition-all"
                                                        >
                                                            "🗑️"
                                                        </button>
                                                    </div>
                                                </div>

                                                {/* Row 2: meta chips indented under the expand button */}
                                                <div class="flex flex-wrap items-center gap-1.5 pl-8">
                                                    {/* Invitation code */}
                                                    {
                                                        let (copied, set_copied) = create_signal(false);
                                                        let code_for_display = guest.invitation_code.chars().take(8).collect::<String>();
                                                        view! {
                                                            <button
                                                                on:click={
                                                                    let code = guest.invitation_code.clone();
                                                                    move |_| {
                                                                        let win = window();
                                                                        if let Ok(origin) = win.location().origin() {
                                                                            let invitation_url = format!("{}/invitation?code={}", origin, code);
                                                                            let clipboard = win.navigator().clipboard();
                                                                            let _ = clipboard.write_text(&invitation_url);
                                                                            set_copied.set(true);
                                                                            set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
                                                                        }
                                                                    }
                                                                }
                                                                class=move || if copied.get() {
                                                                    "px-2 py-0.5 bg-green-100 text-green-800 text-xs font-mono font-semibold rounded border border-green-300 cursor-pointer transition-all"
                                                                } else {
                                                                    "px-2 py-0.5 bg-gray-100 text-gray-600 text-xs font-mono font-semibold rounded border border-gray-200 hover:bg-primary-50 hover:border-primary-300 cursor-pointer transition-all"
                                                                }
                                                                title="Click to copy invitation link"
                                                            >
                                                                {move || if copied.get() { "✓ Copied".to_string() } else { format!("{}…", code_for_display) }}
                                                            </button>
                                                        }
                                                    }

                                                    {/* Party size */}
                                                    <span class="px-2 py-0.5 bg-blue-50 text-blue-700 text-xs font-semibold rounded border border-blue-200" title="Party size">
                                                        {"👥 "}{guest_count}
                                                    </span>

                                                    {/* Location */}
                                                    <span class="px-2 py-0.5 bg-gray-100 text-gray-600 text-xs font-semibold rounded border border-gray-200">
                                                        {guest.locations.iter().map(|l| match l.as_str() {
                                                            "sardinia" => "🇮🇹",
                                                            "tunisia"  => "🇹🇳",
                                                            "nice"     => "🇫🇷",
                                                            _          => "📍",
                                                        }).collect::<Vec<_>>().join(" ")}
                                                    </span>

                                                    {/* Language */}
                                                    <span class="px-2 py-0.5 bg-sky-50 text-sky-700 text-xs font-semibold rounded border border-sky-200">
                                                        {match guest.default_language.as_str() {
                                                            "en" => "🇬🇧".to_string(),
                                                            "fr" => "🇫🇷".to_string(),
                                                            "it" => "🇮🇹".to_string(),
                                                            other => other.to_string(),
                                                        }}
                                                    </span>

                                                    {/* Invited by */}
                                                    <span class="px-2 py-0.5 bg-purple-50 text-purple-700 text-xs font-semibold rounded border border-purple-200">
                                                        {
                                                            let labels: Vec<&str> = guest.invited_by.iter().map(|e| match e.as_str() {
                                                                "mauro.sardara@gmail.com" => "Mauro",
                                                                "munaamamu0@gmail.com" => "Muna",
                                                                other => other,
                                                            }).collect();
                                                            if labels.is_empty() { "—".to_string() } else { labels.join(", ") }
                                                        }
                                                    </span>

                                                    {/* Invitation sent toggle */}
                                                    <button
                                                        on:click={
                                                            let guest_id = guest_id_for_invite.clone();
                                                            move |_| {
                                                                let guest_id = guest_id.clone();
                                                                let new_value = !invitation_sent.get_untracked();
                                                                set_invitation_sent.set(new_value);
                                                                spawn_local(async move {
                                                                    let update = GuestGroupUpdate {
                                                                        name: None,
                                                                        email: None,
                                                                        party_size: None,
                                                                        locations: None,
                                                                        default_language: None,
                                                                        additional_notes: None,
                                                                        invitation_sent: Some(new_value),
                                                                        invited_by: None,
                                                                    };
                                                                    if let Err(e) = admin_context.authenticated_client().update_guest_group(&guest_id, &update).await {
                                                                        set_invitation_sent.set(!new_value);
                                                                        set_error.set(Some(format!("Failed to update invitation status: {}", e)));
                                                                    }
                                                                });
                                                            }
                                                        }
                                                        class=move || if invitation_sent.get() {
                                                            "px-2 py-0.5 text-xs font-semibold rounded border bg-green-100 text-green-700 border-green-300 hover:bg-green-200 cursor-pointer transition-all"
                                                        } else {
                                                            "px-2 py-0.5 text-xs font-semibold rounded border bg-gray-100 text-gray-500 border-gray-200 hover:bg-gray-200 cursor-pointer transition-all"
                                                        }
                                                        title="Click to toggle invitation status"
                                                    >
                                                        {move || if invitation_sent.get() { "✉️ Sent" } else { "✉️ —" }}
                                                    </button>
                                                </div>
                                            </div>

                                            {/* Expanded Guest List */}
                                            {move || if expanded_guest.get().as_ref() == Some(&guest_id_for_expand2) {
                                                let guest_notes = guest.additional_notes.clone();
                                                view! {
                                                    <div class="bg-gray-50 border-t border-gray-200 px-4 py-3">
                                                        {/* Additional Notes */}
                                                        {guest_notes.as_ref().and_then(|notes| {
                                                            if !notes.trim().is_empty() {
                                                                Some(view! {
                                                                    <div class="flex items-start gap-2 mb-3 bg-amber-50 border border-amber-200 rounded p-2">
                                                                        <span class="text-sm flex-shrink-0">"📝"</span>
                                                                        <p class="text-xs text-amber-800 whitespace-pre-wrap">{notes.clone()}</p>
                                                                    </div>
                                                                })
                                                            } else {
                                                                None
                                                            }
                                                        })}

                                                        {move || {
                                                            let invitees_list = guest_invitees.get();
                                                            if invitees_list.is_empty() {
                                                                view! {
                                                                    <p class="text-xs text-gray-400 italic py-2">"No guests added yet"</p>
                                                                }.into_view()
                                                            } else {
                                                                view! {
                                                                    <table class="w-full text-xs border-collapse">
                                                                        <thead>
                                                                            <tr class="border-b border-gray-200 text-gray-500 uppercase tracking-wide">
                                                                                <th class="text-left py-1 pr-3 font-semibold">"Name"</th>
                                                                                <th class="text-left py-1 pr-3 font-semibold">"Age"</th>
                                                                                <th class="text-left py-1 pr-3 font-semibold">"Attending"</th>
                                                                                <th class="text-left py-1 font-semibold">"Dietary"</th>
                                                                            </tr>
                                                                        </thead>
                                                                        <tbody>
                                                                            {invitees_list.into_iter().map(|invitee| {
                                                                                let dietary_badges = invitee.dietary_preferences.as_badges();
                                                                                let other_badge = invitee.dietary_preferences.other_badge();
                                                                                let has_dietary = invitee.dietary_preferences.has_any();
                                                                                let attending_locs = invitee.attending_locations.clone();
                                                                                let has_attending = !attending_locs.is_empty();
                                                                                let age_display = invitee.age_category.display_name().to_string();

                                                                                view! {
                                                                                    <tr class="border-b border-gray-100 last:border-0 hover:bg-white transition-colors">
                                                                                        <td class="py-1.5 pr-3 font-medium text-gray-900">{invitee.name}</td>
                                                                                        <td class="py-1.5 pr-3 text-gray-500">{age_display}</td>
                                                                                        <td class="py-1.5 pr-3">
                                                                                            {if has_attending {
                                                                                                view! {
                                                                                                    <div class="flex flex-wrap gap-1">
                                                                                                        {attending_locs.iter().filter_map(|loc| {
                                                                                                            Location::from_str(loc).map(|l| view! { <LocationBadge location=l /> })
                                                                                                        }).collect::<Vec<_>>()}
                                                                                                    </div>
                                                                                                }.into_view()
                                                                                            } else {
                                                                                                view! { <span class="text-gray-300 italic">"—"</span> }.into_view()
                                                                                            }}
                                                                                        </td>
                                                                                        <td class="py-1.5">
                                                                                            {if has_dietary {
                                                                                                view! {
                                                                                                    <div class="flex flex-wrap gap-1">
                                                                                                        {dietary_badges.iter().map(|(label, css)| view! {
                                                                                                            <span class={format!("px-1.5 py-0.5 text-xs font-semibold rounded-full border {}", css)}>{*label}</span>
                                                                                                        }).collect::<Vec<_>>()}
                                                                                                        {other_badge.map(|(label, css)| view! {
                                                                                                            <span class={format!("px-1.5 py-0.5 text-xs font-semibold rounded-full border {}", css)}>{label}</span>
                                                                                                        })}
                                                                                                    </div>
                                                                                                }.into_view()
                                                                                            } else {
                                                                                                view! { <span class="text-gray-300 italic">"—"</span> }.into_view()
                                                                                            }}
                                                                                        </td>
                                                                                    </tr>
                                                                                }
                                                                            }).collect::<Vec<_>>()}
                                                                        </tbody>
                                                                    </table>
                                                                }.into_view()
                                                            }
                                                        }}
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

            </Show>

            {/* ── All Guests tab ── */}
            <Show when=move || active_tab.get() == "all" fallback=|| ()>
                <div>
                    {/* Filters */}
                    <div class=FILTER_SECTION>
                        <div class=GRID_4_COLS>
                            <div>
                                <label class=FORM_LABEL>"Search"</label>
                                <input
                                    type="text"
                                    placeholder="Search by name or group..."
                                    class=FORM_INPUT
                                    on:input=move |ev| set_all_guests_search.set(event_target_value(&ev))
                                    prop:value=move || all_guests_search.get()
                                />
                            </div>
                            <div>
                                <label class=FORM_LABEL>"Filter by Location"</label>
                                <div class="flex flex-col gap-1 mt-1">
                                    <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                        <input type="checkbox" class="w-4 h-4 text-secondary-600 rounded"
                                            prop:checked=move || ag_loc_sardinia.get()
                                            on:change=move |ev| set_ag_loc_sardinia.set(event_target_checked(&ev)) />
                                        "🇮🇹 Sardinia"
                                    </label>
                                    <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                        <input type="checkbox" class="w-4 h-4 text-secondary-600 rounded"
                                            prop:checked=move || ag_loc_tunisia.get()
                                            on:change=move |ev| set_ag_loc_tunisia.set(event_target_checked(&ev)) />
                                        "🇹🇳 Tunisia"
                                    </label>
                                    <label class="flex items-center gap-2 cursor-pointer text-sm text-gray-700">
                                        <input type="checkbox" class="w-4 h-4 text-secondary-600 rounded"
                                            prop:checked=move || ag_loc_nice.get()
                                            on:change=move |ev| set_ag_loc_nice.set(event_target_checked(&ev)) />
                                        "🇫🇷 Nice"
                                    </label>
                                </div>
                            </div>
                            <div>
                                <label class=FORM_LABEL>"Filter by Invitation"</label>
                                <select class=FORM_SELECT
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_ag_invitation_filter.set(match value.as_str() {
                                            "sent"     => Some(true),
                                            "not_sent" => Some(false),
                                            _          => None,
                                        });
                                    }
                                >
                                    <option value="">"All"</option>
                                    <option value="sent">"Sent"</option>
                                    <option value="not_sent">"Not Sent"</option>
                                </select>
                            </div>
                            <div>
                                <label class=FORM_LABEL>"Filter by Invited By"</label>
                                <select class=FORM_SELECT
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_ag_invited_by_filter.set(if value.is_empty() { None } else { Some(value) });
                                    }
                                >
                                    <option value="">"All"</option>
                                    <option value="mauro.sardara@gmail.com">"Mauro"</option>
                                    <option value="munaamamu0@gmail.com">"Muna"</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="mt-4">
                    {move || if all_guests_loading.get() {
                        view! {
                            <div class=LOADING_CONTAINER>
                                <div class=LOADING_SPINNER></div>
                            </div>
                        }.into_view()
                    } else {
                        let name_lookup    = group_name_lookup();
                        let inv_by_lookup  = group_invited_by_lookup();
                        let locs_lookup    = group_locations_lookup();
                        let inv_sent_lookup = group_invitation_sent_lookup();
                        let query          = all_guests_search.get().to_lowercase();
                        let col            = sort_col.get();
                        let asc            = sort_asc.get();

                        let filter_sardinia  = ag_loc_sardinia.get();
                        let filter_tunisia   = ag_loc_tunisia.get();
                        let filter_nice      = ag_loc_nice.get();
                        let inv_filter       = ag_invitation_filter.get();
                        let invited_filter   = ag_invited_by_filter.get();

                        let mut list: Vec<_> = all_guests.get().into_iter().filter(|g| {
                            let group_name = name_lookup.get(&g.guest_group_id).map(|s| s.as_str()).unwrap_or("");
                            let group_locs = locs_lookup.get(&g.guest_group_id);
                            let group_inv_sent = inv_sent_lookup.get(&g.guest_group_id).copied().unwrap_or(false);
                            let group_inv_by = inv_by_lookup.get(&g.guest_group_id);

                            let matches_search = query.is_empty()
                                || g.name.to_lowercase().contains(&query)
                                || group_name.to_lowercase().contains(&query);

                            let any_loc = filter_sardinia || filter_tunisia || filter_nice;
                            let matches_location = !any_loc
                                || (filter_sardinia && group_locs.map_or(false, |l| l.contains(&"sardinia".to_string())))
                                || (filter_tunisia  && group_locs.map_or(false, |l| l.contains(&"tunisia".to_string())))
                                || (filter_nice     && group_locs.map_or(false, |l| l.contains(&"nice".to_string())));

                            let matches_invitation = inv_filter.is_none_or(|sent| group_inv_sent == sent);

                            let matches_invited_by = invited_filter.as_ref()
                                .is_none_or(|email| group_inv_by.map_or(false, |v| v.contains(email)));

                            matches_search && matches_location && matches_invitation && matches_invited_by
                        }).collect();

                        // Sort
                        list.sort_by(|a, b| {
                            let ord = match col {
                                "group"      => {
                                    let na = name_lookup.get(&a.guest_group_id).map(|s| s.as_str()).unwrap_or("");
                                    let nb = name_lookup.get(&b.guest_group_id).map(|s| s.as_str()).unwrap_or("");
                                    na.cmp(nb)
                                }
                                "age"        => a.age_category.display_name().cmp(b.age_category.display_name()),
                                "attending"  => a.attending_locations.join(",").cmp(&b.attending_locations.join(",")),
                                "invited_by" => {
                                    let ia = inv_by_lookup.get(&a.guest_group_id).map(|v| v.join(", ")).unwrap_or_default();
                                    let ib = inv_by_lookup.get(&b.guest_group_id).map(|v| v.join(", ")).unwrap_or_default();
                                    ia.cmp(&ib)
                                }
                                _            => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                            };
                            if asc { ord } else { ord.reverse() }
                        });

                        // Helper: header cell that toggles sort
                        let th = move |label: &'static str, key: &'static str| {
                            let active = sort_col.get() == key;
                            let indicator = if active {
                                if sort_asc.get() { " ▲" } else { " ▼" }
                            } else { "" };
                            view! {
                                <th
                                    class=move || {
                                        let base = "text-left px-3 py-2 font-semibold cursor-pointer select-none hover:text-gray-800 whitespace-nowrap";
                                        if sort_col.get() == key {
                                            format!("{} text-secondary-700", base)
                                        } else {
                                            format!("{} text-gray-500", base)
                                        }
                                    }
                                    on:click=move |_| {
                                        if sort_col.get() == key {
                                            set_sort_asc.update(|v| *v = !*v);
                                        } else {
                                            set_sort_col.set(key);
                                            set_sort_asc.set(true);
                                        }
                                    }
                                >
                                    {format!("{}{}", label, indicator)}
                                </th>
                            }
                        };

                        if list.is_empty() {
                            view! { <p class="text-sm text-gray-400 italic py-4">"No guests found."</p> }.into_view()
                        } else {
                            view! {
                                {
                                    let group_count = list.iter()
                                        .map(|g| &g.guest_group_id)
                                        .collect::<std::collections::HashSet<_>>()
                                        .len();
                                    let attending_count = list.iter()
                                        .filter(|g| !g.attending_locations.is_empty())
                                        .count();
                                    view! {
                                        <div class="text-xs text-gray-500 mb-2 flex flex-wrap gap-3">
                                            <span>
                                                <span class="font-semibold text-gray-700">{list.len()}</span>
                                                " guests across "
                                                <span class="font-semibold text-gray-700">{group_count}</span>
                                                " groups"
                                            </span>
                                            <span class="text-green-700">
                                                "✓ "
                                                <span class="font-semibold">{attending_count}</span>
                                                " confirmed"
                                            </span>
                                            <span class="text-gray-400">
                                                "◌ "
                                                <span class="font-semibold">{list.len() - attending_count}</span>
                                                " pending"
                                            </span>
                                        </div>
                                    }
                                }
                                <div class="overflow-x-auto">
                                <table class="w-full text-xs border-collapse bg-white rounded-lg overflow-hidden shadow">
                                    <thead>
                                        <tr class="bg-gray-50 border-b border-gray-200 uppercase tracking-wide">
                                            {th("Name", "name")}
                                            {th("Group", "group")}
                                            {th("Age", "age")}
                                            {th("Attending", "attending")}
                                            {th("Dietary", "dietary")}
                                            {th("Invited By", "invited_by")}
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {list.into_iter().map(|g| {
                                            let group_name    = name_lookup.get(&g.guest_group_id).cloned().unwrap_or_default();
                                            let invited_by_labels = inv_by_lookup
                                                .get(&g.guest_group_id)
                                                .map(|v| v.iter().map(|e| match e.as_str() {
                                                    "mauro.sardara@gmail.com" => "Mauro",
                                                    "munaamamu0@gmail.com"    => "Muna",
                                                    other                     => other,
                                                }).collect::<Vec<_>>().join(", "))
                                                .unwrap_or_default();
                                            let dietary_badges = g.dietary_preferences.as_badges();
                                            let other_badge    = g.dietary_preferences.other_badge();
                                            let has_dietary    = g.dietary_preferences.has_any();
                                            let attending_locs = g.attending_locations.clone();
                                            let has_attending  = !attending_locs.is_empty();
                                            let age_display    = g.age_category.display_name().to_string();

                                            view! {
                                                <tr class="border-b border-gray-100 last:border-0 hover:bg-gray-50 transition-colors">
                                                    <td class="px-3 py-2 font-medium text-gray-900 whitespace-nowrap">{g.name}</td>
                                                    <td class="px-3 py-2 text-gray-500 whitespace-nowrap">{group_name}</td>
                                                    <td class="px-3 py-2 text-gray-500 whitespace-nowrap">{age_display}</td>
                                                    <td class="px-3 py-2">
                                                        {if has_attending {
                                                            view! {
                                                                <div class="flex flex-wrap gap-1">
                                                                    {attending_locs.iter().filter_map(|loc| {
                                                                        Location::from_str(loc).map(|l| view! { <LocationBadge location=l /> })
                                                                    }).collect::<Vec<_>>()}
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span class="text-gray-300 italic">"—"</span> }.into_view()
                                                        }}
                                                    </td>
                                                    <td class="px-3 py-2">
                                                        {if has_dietary {
                                                            view! {
                                                                <div class="flex flex-wrap gap-1">
                                                                    {dietary_badges.iter().map(|(label, css)| view! {
                                                                        <span class={format!("px-1.5 py-0.5 text-xs font-semibold rounded-full border {}", css)}>{*label}</span>
                                                                    }).collect::<Vec<_>>()}
                                                                    {other_badge.map(|(label, css)| view! {
                                                                        <span class={format!("px-1.5 py-0.5 text-xs font-semibold rounded-full border {}", css)}>{label}</span>
                                                                    })}
                                                                </div>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span class="text-gray-300 italic">"—"</span> }.into_view()
                                                        }}
                                                    </td>
                                                    <td class="px-3 py-2 text-gray-500 whitespace-nowrap">
                                                        {if invited_by_labels.is_empty() {
                                                            view! { <span class="text-gray-300 italic">"—"</span> }.into_view()
                                                        } else {
                                                            view! { <span>{invited_by_labels}</span> }.into_view()
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                                </div>
                            }.into_view()
                        }
                    }}
                    </div>
                </div>
            </Show>

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
