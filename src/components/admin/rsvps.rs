use crate::components::common::StatCard;
use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Guest, GuestGroup, Location, RsvpStatus};
use leptos::*;
use std::collections::HashMap;

type TaggedGuests = Signal<Vec<(Guest, RsvpStatus)>>;

#[component]
pub fn RsvpManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (guest_groups, set_guest_groups) = create_signal::<Vec<GuestGroup>>(Vec::new());
    let (all_guests, set_all_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Load guest data
    let load_data = create_action(move |_: &()| async move {
        set_loading.set(true);
        set_error.set(None);

        let groups_result = admin_context
            .authenticated_client()
            .get_all_guest_groups()
            .await;
        let all_guests_result = admin_context.authenticated_client().get_all_guests().await;

        match (groups_result, all_guests_result) {
            (Ok(groups), Ok(guests)) => {
                set_guest_groups.set(groups);
                set_all_guests.set(guests);
                set_loading.set(false);
            }
            _ => {
                set_error.set(Some("Failed to load data".to_string()));
                set_loading.set(false);
            }
        }
    });

    // Load data on mount (fire once, not inside a reactive effect)
    request_animation_frame(move || {
        load_data.dispatch(());
    });

    // Lookup: group_id -> rsvp_submitted
    let rsvp_lookup = Signal::derive(move || {
        guest_groups
            .get()
            .into_iter()
            .map(|g| (g.id, g.rsvp_submitted))
            .collect::<HashMap<String, bool>>()
    });

    // Lookup: group_id -> invited locations
    let group_locs_lookup = Signal::derive(move || {
        guest_groups
            .get()
            .into_iter()
            .map(|g| (g.id, g.locations))
            .collect::<HashMap<String, Vec<String>>>()
    });

    // Per-location tagged lists: (Guest, RsvpStatus)
    // Confirmed = attending; Pending = group not yet submitted; Declined = submitted but not attending
    let sardinia_tagged = Signal::derive(move || {
        build_tagged_list(
            all_guests.get(),
            &rsvp_lookup.get(),
            &group_locs_lookup.get(),
            "sardinia",
        )
    });
    let tunisia_tagged = Signal::derive(move || {
        build_tagged_list(
            all_guests.get(),
            &rsvp_lookup.get(),
            &group_locs_lookup.get(),
            "tunisia",
        )
    });
    let nice_tagged = Signal::derive(move || {
        build_tagged_list(
            all_guests.get(),
            &rsvp_lookup.get(),
            &group_locs_lookup.get(),
            "nice",
        )
    });

    // Summary card totals
    let totals = move || {
        let guests = all_guests.get();

        let total_guests = guests.len() as i32;
        let sardinia_count = sardinia_tagged
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Confirmed)
            .count() as i32;
        let tunisia_count = tunisia_tagged
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Confirmed)
            .count() as i32;
        let nice_count = nice_tagged
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Confirmed)
            .count() as i32;

        // Dietary counts from confirmed guests only
        let confirmed: Vec<&Guest> = guests
            .iter()
            .filter(|g| !g.attending_locations.is_empty())
            .collect();
        let vegetarian = confirmed
            .iter()
            .filter(|g| g.dietary_preferences.vegetarian)
            .count() as i32;
        let vegan = confirmed
            .iter()
            .filter(|g| g.dietary_preferences.vegan)
            .count() as i32;
        let halal = confirmed
            .iter()
            .filter(|g| g.dietary_preferences.halal)
            .count() as i32;
        let no_pork = confirmed
            .iter()
            .filter(|g| g.dietary_preferences.no_pork)
            .count() as i32;
        let gluten_free = confirmed
            .iter()
            .filter(|g| g.dietary_preferences.gluten_free)
            .count() as i32;
        let other = confirmed
            .iter()
            .filter(|g| !g.dietary_preferences.other.is_empty())
            .count() as i32;

        (
            total_guests,
            sardinia_count,
            tunisia_count,
            nice_count,
            vegetarian,
            vegan,
            halal,
            no_pork,
            gluten_free,
            other,
        )
    };

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "Guest Management"
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
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
                <StatCard
                    icon="👥"
                    title="Total Guests"
                    value=Signal::derive(move || totals().0)
                    color="blue"
                />
                <StatCard
                    icon="🇮🇹"
                    title="Sardinia"
                    value=Signal::derive(move || totals().1)
                    color="blue"
                />
                <StatCard
                    icon="🇹🇳"
                    title="Tunisia"
                    value=Signal::derive(move || totals().2)
                    color="blue"
                />
                <StatCard
                    icon="🇫🇷"
                    title="Nice"
                    value=Signal::derive(move || totals().3)
                    color="blue"
                />
                <StatCard
                    icon="🥗"
                    title="Vegetarian"
                    value=Signal::derive(move || totals().4)
                    color="yellow"
                />
                <StatCard
                    icon="🌱"
                    title="Vegan"
                    value=Signal::derive(move || totals().5)
                    color="purple"
                />
                <StatCard
                    icon="☪️"
                    title="Halal"
                    value=Signal::derive(move || totals().6)
                    color="indigo"
                />
                <StatCard
                    icon="🚫🐷"
                    title="No Pork"
                    value=Signal::derive(move || totals().7)
                    color="pink"
                />
                <StatCard
                    icon="🌾"
                    title="Gluten-Free"
                    value=Signal::derive(move || totals().8)
                    color="amber"
                />
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class=LOADING_CONTAINER>
                            <div class=LOADING_SPINNER></div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6 mt-8">
                            {
                                let locations_and_signals: Vec<(Location, TaggedGuests)> = vec![
                                    (Location::Sardinia, sardinia_tagged),
                                    (Location::Tunisia,  tunisia_tagged),
                                    (Location::Nice,     nice_tagged),
                                ];
                                locations_and_signals.into_iter().map(|(loc, sig)| {
                                    let (hdr_bg, hdr_border, title_txt, count_txt) = loc.table_header_colors();
                                    let title = Box::leak(format!("{} Guests", loc.display_name()).into_boxed_str()) as &'static str;
                                    let flag = loc.flag_emoji();
                                    view! {
                                        <LocationGuestTable
                                            title=title
                                            flag=flag
                                            header_bg=hdr_bg
                                            header_border=hdr_border
                                            title_text=title_txt
                                            count_text=count_txt
                                            guests=sig
                                        />
                                    }
                                }).collect_view()
                            }
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

/// Builds a tagged guest list for a single location.
/// Each entry is (Guest, RsvpStatus) where:
/// - Confirmed: guest's attending_locations includes this location
/// - Pending:   group has not yet submitted RSVP
/// - Declined:  group submitted but guest is not attending this location
fn build_tagged_list(
    guests: Vec<Guest>,
    rsvp: &HashMap<String, bool>,
    locs: &HashMap<String, Vec<String>>,
    location: &str,
) -> Vec<(Guest, RsvpStatus)> {
    let mut result: Vec<(Guest, RsvpStatus)> = guests
        .into_iter()
        .filter(|g| {
            locs.get(&g.guest_group_id)
                .is_some_and(|l| l.contains(&location.to_string()))
        })
        .map(|g| {
            let submitted = rsvp.get(&g.guest_group_id).copied().unwrap_or(false);
            let attending = g.attending_locations.contains(&location.to_string());
            let status = if attending {
                RsvpStatus::Confirmed
            } else if submitted {
                RsvpStatus::Declined
            } else {
                RsvpStatus::Pending
            };
            (g, status)
        })
        .collect();

    // Sort: Confirmed first, then Pending, then Declined
    result.sort_by_key(|(_, s)| match s {
        RsvpStatus::Confirmed => 0,
        RsvpStatus::Pending => 1,
        RsvpStatus::Declined => 2,
        RsvpStatus::Partial => 3,
    });

    result
}

/// A reusable guest-list table for a single location.
#[component]
fn LocationGuestTable(
    title: &'static str,
    flag: &'static str,
    header_bg: &'static str,
    header_border: &'static str,
    title_text: &'static str,
    count_text: &'static str,
    guests: TaggedGuests,
) -> impl IntoView {
    let total_count = move || guests.get().len();
    let confirmed_count = move || {
        guests
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Confirmed)
            .count()
    };
    let pending_count = move || {
        guests
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Pending)
            .count()
    };
    let declined_count = move || {
        guests
            .get()
            .iter()
            .filter(|(_, s)| *s == RsvpStatus::Declined)
            .count()
    };

    view! {
        <div class="bg-white rounded-lg shadow-md overflow-hidden">
            <div class={format!("{} px-6 py-4 border-b {}", header_bg, header_border)}>
                <h3 class={format!("text-lg font-semibold {} flex items-center gap-2", title_text)}>
                    {flag} " " {title}
                    <span class={format!("text-sm font-normal {}", count_text)}>
                        "(" {move || total_count()} " total)"
                    </span>
                </h3>
                <div class={format!("flex items-center gap-3 mt-1 text-sm {}", count_text)}>
                    <span>"✓ " {move || confirmed_count()} " confirmed"</span>
                    <span class="opacity-50">"·"</span>
                    <span>"⏳ " {move || pending_count()} " pending"</span>
                    <span class="opacity-50">"·"</span>
                    <span>"✗ " {move || declined_count()} " declined"</span>
                </div>
            </div>
            <div class="p-6">
                <div class="overflow-x-auto">
                    <table class="min-w-full divide-y divide-gray-200">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">
                                    "Name"
                                </th>
                                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">
                                    "Status"
                                </th>
                                <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">
                                    "Dietary"
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200">
                            {move || guests.get().into_iter().map(|(guest, status)| {
                                let dietary = guest.dietary_preferences.format_display();
                                let (status_class, status_label) = match status {
                                    RsvpStatus::Confirmed => (
                                        "px-1.5 py-0.5 text-xs font-semibold rounded bg-green-100 text-green-700",
                                        "✓ Confirmed",
                                    ),
                                    RsvpStatus::Pending => (
                                        "px-1.5 py-0.5 text-xs font-semibold rounded bg-amber-100 text-amber-700",
                                        "⏳ Pending",
                                    ),
                                    RsvpStatus::Declined => (
                                        "px-1.5 py-0.5 text-xs font-semibold rounded bg-red-100 text-red-600",
                                        "✗ Declined",
                                    ),
                                    RsvpStatus::Partial => (
                                        "px-1.5 py-0.5 text-xs font-semibold rounded bg-gray-100 text-gray-600",
                                        "~ Partial",
                                    ),
                                };
                                let row_class = match status {
                                    RsvpStatus::Confirmed => "hover:bg-gray-50",
                                    RsvpStatus::Pending   => "hover:bg-amber-50 opacity-70",
                                    RsvpStatus::Declined  => "hover:bg-red-50 opacity-50",
                                    RsvpStatus::Partial   => "hover:bg-gray-50",
                                };
                                view! {
                                    <tr class=row_class>
                                        <td class="px-4 py-2 text-sm text-gray-900">
                                            {guest.name}
                                        </td>
                                        <td class="px-4 py-2">
                                            <span class=status_class>{status_label}</span>
                                        </td>
                                        <td class="px-4 py-2 text-sm text-gray-600">
                                            {dietary}
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
