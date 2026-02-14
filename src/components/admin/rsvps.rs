use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::{Guest, GuestGroup};
use leptos::*;

#[component]
pub fn RsvpManagement() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (_guest_groups, set_guest_groups) = create_signal::<Vec<GuestGroup>>(Vec::new());
    let (all_guests, set_all_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (sardinia_guests, set_sardinia_guests) = create_signal::<Vec<Guest>>(Vec::new());
    let (tunisia_guests, set_tunisia_guests) = create_signal::<Vec<Guest>>(Vec::new());
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

        // Load location-specific guest lists
        let sardinia_result = admin_context
            .authenticated_client()
            .get_all_guests_for_location("sardinia")
            .await;

        let tunisia_result = admin_context
            .authenticated_client()
            .get_all_guests_for_location("tunisia")
            .await;

        match (groups_result, sardinia_result, tunisia_result) {
            (Ok(groups), Ok(sardinia_list), Ok(tunisia_list)) => {
                set_guest_groups.set(groups);

                // Combine sardinia and tunisia guests for total counts
                let mut all = sardinia_list.clone();
                all.extend(tunisia_list.clone());
                set_all_guests.set(all);

                set_sardinia_guests.set(sardinia_list);
                set_tunisia_guests.set(tunisia_list);
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

        let total_guests = guests.len() as i32;
        let sardinia_count = sardinia.len() as i32;
        let tunisia_count = tunisia.len() as i32;

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
                <SummaryCard
                    icon="👥"
                    label="Total Guests"
                    value=move || totals().0
                    color="blue"
                />
                <SummaryCard
                    icon="🇮🇹"
                    label="Sardinia"
                    value=move || totals().1
                    color="blue"
                />
                <SummaryCard
                    icon="🇹🇳"
                    label="Tunisia"
                    value=move || totals().2
                    color="green"
                />
                <SummaryCard
                    icon="🥗"
                    label="Vegetarian"
                    value=move || totals().3
                    color="yellow"
                />
                <SummaryCard
                    icon="🌱"
                    label="Vegan"
                    value=move || totals().4
                    color="purple"
                />
                <SummaryCard
                    icon="☪️"
                    label="Halal"
                    value=move || totals().5
                    color="indigo"
                />
                <SummaryCard
                    icon="🚫🐷"
                    label="No Pork"
                    value=move || totals().6
                    color="pink"
                />
                <SummaryCard
                    icon="🌾"
                    label="Gluten-Free"
                    value=move || totals().7
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
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-8">
                            {/* Sardinia Guests */}
                            <div class="bg-white rounded-lg shadow-md overflow-hidden">
                                <div class="bg-blue-50 px-6 py-4 border-b border-blue-200">
                                    <h3 class="text-lg font-semibold text-blue-900 flex items-center gap-2">
                                        "🇮🇹 Sardinia Guests"
                                        <span class="text-sm font-normal text-blue-700">
                                            "(" {move || sardinia_guests.get().len()} " guests)"
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
                                                {move || sardinia_guests.get().into_iter().map(|guest| {
                                                    let dietary = format_dietary(&guest);

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

                            {/* Tunisia Guests */}
                            <div class="bg-white rounded-lg shadow-md overflow-hidden">
                                <div class="bg-green-50 px-6 py-4 border-b border-green-200">
                                    <h3 class="text-lg font-semibold text-green-900 flex items-center gap-2">
                                        "🇹🇳 Tunisia Guests"
                                        <span class="text-sm font-normal text-green-700">
                                            "(" {move || tunisia_guests.get().len()} " guests)"
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
                                                {move || tunisia_guests.get().into_iter().map(|guest| {
                                                    let dietary = format_dietary(&guest);

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
        "indigo" => "bg-indigo-50",
        "pink" => "bg-pink-50",
        "amber" => "bg-amber-50",
        "orange" => "bg-orange-50",
        _ => "bg-gray-50",
    };

    let text_color = match color {
        "green" => "text-green-600",
        "blue" => "text-blue-600",
        "yellow" => "text-yellow-600",
        "purple" => "text-purple-600",
        "indigo" => "text-indigo-600",
        "pink" => "text-pink-600",
        "amber" => "text-amber-600",
        "orange" => "text-orange-600",
        _ => "text-gray-600",
    };

    view! {
        <div class={format!("rounded-lg shadow-md p-4 {}", bg_color)}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-xs font-medium text-gray-600 mb-1">{label}</p>
                    <p class={format!("text-2xl font-bold {}", text_color)}>{move || value()}</p>
                </div>
                <div class="text-3xl">{icon}</div>
            </div>
        </div>
    }
}

fn format_dietary(guest: &Guest) -> String {
    let mut items = Vec::new();

    if guest.dietary_preferences.vegetarian {
        items.push("🥗 Vegetarian".to_string());
    }
    if guest.dietary_preferences.vegan {
        items.push("🌱 Vegan".to_string());
    }
    if guest.dietary_preferences.halal {
        items.push("☪️ Halal".to_string());
    }
    if guest.dietary_preferences.no_pork {
        items.push("🚫🐷 No Pork".to_string());
    }
    if guest.dietary_preferences.gluten_free {
        items.push("🌾 Gluten-Free".to_string());
    }
    if !guest.dietary_preferences.other.is_empty() {
        items.push(format!("📝 {}", guest.dietary_preferences.other));
    }

    if items.is_empty() {
        "-".to_string()
    } else {
        items.join(", ")
    }
}
