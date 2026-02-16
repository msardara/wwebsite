use crate::components::common::StatCard;
use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Guest, GuestGroup, Location};
use leptos::*;

#[component]
pub fn RsvpManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (_guest_groups, set_guest_groups) = create_signal::<Vec<GuestGroup>>(Vec::new());
    let (all_guests, set_all_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (sardinia_guests, set_sardinia_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (tunisia_guests, set_tunisia_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (nice_guests, set_nice_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Load guest data
    let load_data = create_action(move |_: &()| async move {
        set_loading.set(true);
        set_error.set(None);

        // Load all guest groups
        let groups_result = admin_context
            .authenticated_client()
            .get_all_guest_groups()
            .await;

        // Load all guests in a single query
        let all_guests_result = admin_context.authenticated_client().get_all_guests().await;

        match (groups_result, all_guests_result) {
            (Ok(groups), Ok(guests)) => {
                set_guest_groups.set(groups);

                // Filter by location client-side
                let sardinia_list: Vec<Guest> = guests
                    .iter()
                    .filter(|g| g.attending_locations.iter().any(|l| l == "sardinia"))
                    .cloned()
                    .collect();
                let tunisia_list: Vec<Guest> = guests
                    .iter()
                    .filter(|g| g.attending_locations.iter().any(|l| l == "tunisia"))
                    .cloned()
                    .collect();
                let nice_list: Vec<Guest> = guests
                    .iter()
                    .filter(|g| g.attending_locations.iter().any(|l| l == "nice"))
                    .cloned()
                    .collect();

                set_all_guests.set(guests);
                set_sardinia_guests.set(sardinia_list);
                set_tunisia_guests.set(tunisia_list);
                set_nice_guests.set(nice_list);
                set_loading.set(false);
            }
            _ => {
                set_error.set(Some("Failed to load data".to_string()));
                set_loading.set(false);
            }
        }
    });

    // Load data on mount
    create_effect(move |_| {
        load_data.dispatch(());
    });

    // Calculate totals
    let totals = move || {
        let guests = all_guests.get();
        let sardinia = sardinia_guests.get();
        let tunisia = tunisia_guests.get();
        let nice = nice_guests.get();

        let total_guests = guests.len() as i32;
        let sardinia_count = sardinia.len() as i32;
        let tunisia_count = tunisia.len() as i32;
        let nice_count = nice.len() as i32;

        let vegetarian: i32 = guests
            .iter()
            .filter(|g| g.dietary_preferences.vegetarian)
            .count() as i32;
        let vegan: i32 = guests
            .iter()
            .filter(|g| g.dietary_preferences.vegan)
            .count() as i32;
        let halal: i32 = guests
            .iter()
            .filter(|g| g.dietary_preferences.halal)
            .count() as i32;
        let no_pork: i32 = guests
            .iter()
            .filter(|g| g.dietary_preferences.no_pork)
            .count() as i32;
        let gluten_free: i32 = guests
            .iter()
            .filter(|g| g.dietary_preferences.gluten_free)
            .count() as i32;
        let other: i32 = guests
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

            {/* Summary Cards — using the shared StatCard component */}
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
                                let locations_and_signals: Vec<(Location, ReadSignal<Vec<Guest>>)> = vec![
                                    (Location::Sardinia, sardinia_guests),
                                    (Location::Tunisia, tunisia_guests),
                                    (Location::Nice, nice_guests),
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

/// A reusable guest-list table for a single location.
///
/// Replaces the three nearly-identical Sardinia / Tunisia / Nice blocks that
/// were previously copy-pasted in this file.
#[component]
fn LocationGuestTable(
    title: &'static str,
    flag: &'static str,
    header_bg: &'static str,
    header_border: &'static str,
    title_text: &'static str,
    count_text: &'static str,
    guests: ReadSignal<Vec<Guest>>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md overflow-hidden">
            <div class={format!("{} px-6 py-4 border-b {}", header_bg, header_border)}>
                <h3 class={format!("text-lg font-semibold {} flex items-center gap-2", title_text)}>
                    {flag} " " {title}
                    <span class={format!("text-sm font-normal {}", count_text)}>
                        "(" {move || guests.get().len()} " guests)"
                    </span>
                </h3>
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
                                    "Dietary"
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200">
                            {move || guests.get().into_iter().map(|guest| {
                                let dietary = guest.dietary_preferences.format_display();

                                view! {
                                    <tr class="hover:bg-gray-50">
                                        <td class="px-4 py-2 text-sm text-gray-900">
                                            {guest.name}
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
