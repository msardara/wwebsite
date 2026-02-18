use super::csv_export;
use crate::components::common::{IconStatCard, StatCard};
use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::AdminStats;
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (stats, set_stats) = create_signal::<Option<AdminStats>>(None);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (export_error, set_export_error) = create_signal::<Option<String>>(None);
    let (exporting_groups, set_exporting_groups) = create_signal(false);
    let (exporting_guests, set_exporting_guests) = create_signal(false);

    // Single action for both initial load and manual refresh
    let load_stats = create_action(move |_: &()| async move {
        set_loading.set(true);
        set_error.set(None);

        match admin_context.authenticated_client().get_admin_stats().await {
            Ok(admin_stats) => {
                set_stats.set(Some(admin_stats));
                set_loading.set(false);
            }
            Err(e) => {
                set_error.set(Some(format!("Failed to load statistics: {}", e)));
                set_loading.set(false);
            }
        }
    });

    // Export guest groups as CSV
    let export_guest_groups = create_action(move |_: &()| async move {
        set_exporting_groups.set(true);
        set_export_error.set(None);

        let client = admin_context.authenticated_client();
        match client.get_all_guest_groups().await {
            Ok(groups) => {
                let csv = csv_export::guest_groups_to_csv(&groups);
                if let Err(e) = csv_export::trigger_csv_download(&csv, "guest_groups.csv") {
                    set_export_error.set(Some(format!("Download failed: {}", e)));
                }
            }
            Err(e) => {
                set_export_error.set(Some(format!("Failed to fetch guest groups: {}", e)));
            }
        }

        set_exporting_groups.set(false);
    });

    // Export guests as CSV
    let export_guests = create_action(move |_: &()| async move {
        set_exporting_guests.set(true);
        set_export_error.set(None);

        let client = admin_context.authenticated_client();

        // Fetch groups first to build a name lookup map
        let groups_result = client.get_all_guest_groups().await;
        let guests_result = client.get_all_guests().await;

        match (groups_result, guests_result) {
            (Ok(groups), Ok(guests)) => {
                let group_lookup: HashMap<String, String> = groups
                    .iter()
                    .map(|g| (g.id.clone(), g.name.clone()))
                    .collect();

                let csv = csv_export::guests_to_csv(&guests, &group_lookup);
                if let Err(e) = csv_export::trigger_csv_download(&csv, "guests.csv") {
                    set_export_error.set(Some(format!("Download failed: {}", e)));
                }
            }
            (Err(e), _) => {
                set_export_error.set(Some(format!("Failed to fetch guest groups: {}", e)));
            }
            (_, Err(e)) => {
                set_export_error.set(Some(format!("Failed to fetch guests: {}", e)));
            }
        }

        set_exporting_guests.set(false);
    });

    // Load statistics on mount (fire once, not inside a reactive effect)
    request_animation_frame(move || {
        load_stats.dispatch(());
    });

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "Dashboard"
                </h2>
                <div class="flex items-center gap-2">
                    <button
                        on:click=move |_| export_guest_groups.dispatch(())
                        class=BUTTON_SECONDARY_INLINE
                        disabled=move || exporting_groups.get()
                    >
                        {move || if exporting_groups.get() { "⏳ Exporting…" } else { "📥 Groups CSV" }}
                    </button>
                    <button
                        on:click=move |_| export_guests.dispatch(())
                        class=BUTTON_SECONDARY_INLINE
                        disabled=move || exporting_guests.get()
                    >
                        {move || if exporting_guests.get() { "⏳ Exporting…" } else { "📥 Guests CSV" }}
                    </button>
                    <button
                        on:click=move |_| load_stats.dispatch(())
                        class=REFRESH_BUTTON
                    >
                        "↻ Refresh"
                    </button>
                </div>
            </div>

            {move || export_error.get().map(|err| view! {
                <div class=ALERT_ERROR>
                    {err}
                </div>
            })}

            {move || {
                if loading.get() {
                    view! {
                        <div class=LOADING_CONTAINER>
                            <div class=LOADING_SPINNER></div>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class=ALERT_ERROR>
                            {err}
                        </div>
                    }.into_view()
                } else if let Some(s) = stats.get() {
                    view! {
                        <div class="space-y-6">
                            {/* Overview Statistics */}
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                                <StatCard
                                    icon="✉️"
                                    title="Total Invited Guests"
                                    value=s.total_guests
                                    color="blue"
                                />
                                <StatCard
                                    icon="✅"
                                    title="Guests Confirmed"
                                    value=s.total_confirmed
                                    color="green"
                                />
                                <StatCard
                                    icon="⏳"
                                    title="Pending Guests"
                                    value=s.pending_rsvps
                                    color="yellow"
                                />
                                <StatCard
                                    icon="👥"
                                    title="Total Guest Groups"
                                    value=s.total_guest_groups
                                    color="purple"
                                />
                            </div>

                            {/* Location Breakdown */}
                            <div class=CARD>
                                <h3 class=SECTION_TITLE>
                                    "Location Breakdown"
                                </h3>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <IconStatCard
                                        icon="🏖️"
                                        label="Sardinia"
                                        count=s.sardinia_guests
                                    />
                                    <IconStatCard
                                        icon="🌴"
                                        label="Tunisia"
                                        count=s.tunisia_guests
                                    />
                                    <IconStatCard
                                        icon="🏔️"
                                        label="Nice"
                                        count=s.nice_guests
                                    />
                                </div>
                                <p class="text-sm text-gray-500 mt-2 italic">
                                    "Note: Guests attending multiple locations are counted in each location."
                                </p>
                            </div>

                            {/* Dietary Restrictions */}
                            <div class=CARD>
                                <h3 class=SECTION_TITLE>
                                    "Dietary Restrictions"
                                </h3>
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <IconStatCard
                                        icon="🥗"
                                        label="Vegetarian"
                                        count=s.vegetarian_count
                                    />
                                    <IconStatCard
                                        icon="🌱"
                                        label="Vegan"
                                        count=s.vegan_count
                                    />
                                    <IconStatCard
                                        icon="☪️"
                                        label="Halal"
                                        count=s.halal_count
                                    />
                                    <IconStatCard
                                        icon="🚫🐷"
                                        label="No Pork"
                                        count=s.no_pork_count
                                    />
                                    <IconStatCard
                                        icon="🌾"
                                        label="Gluten-Free"
                                        count=s.gluten_free_count
                                    />
                                    <IconStatCard
                                        icon="🍽️"
                                        label="Other Dietary"
                                        count=s.other_dietary_count
                                    />
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="text-center text-gray-500 py-12">
                            "No statistics available"
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
