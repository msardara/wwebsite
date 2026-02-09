use crate::contexts::AdminContext;
use crate::styles::*;
use crate::types::AdminStats;
use leptos::*;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let admin_context = use_context::<AdminContext>().expect("AdminContext not found");

    let (stats, set_stats) = create_signal::<Option<AdminStats>>(None);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Load statistics on mount
    let admin_context_clone = admin_context;
    create_effect(move |_| {
        let admin_context = admin_context_clone;
        spawn_local(async move {
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
    });

    view! {
        <div class=ADMIN_CONTAINER>
            <div class=PAGE_HEADER_CONTAINER>
                <h2 class=PAGE_HEADER>
                    "Dashboard"
                </h2>
                <button
                    on:click=move |_| {
                        let admin_context = admin_context;
                        spawn_local(async move {
                            set_loading.set(true);
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
                    }
                    class=REFRESH_BUTTON
                >
                    "↻ Refresh"
                </button>
            </div>

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
                                    icon="📋"
                                    title="Pending Group Invitations"
                                    value=s.both_locations_guests
                                    color="purple"
                                />
                            </div>

                            {/* Location Breakdown */}
                            <div class=CARD>
                                <h3 class=SECTION_TITLE>
                                    "Location Breakdown"
                                </h3>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <LocationCard
                                        icon="🏖️"
                                        location="Sardinia"
                                        count=s.sardinia_guests
                                    />
                                    <LocationCard
                                        icon="🌴"
                                        location="Tunisia"
                                        count=s.tunisia_guests
                                    />
                                </div>
                                <p class="text-sm text-gray-500 mt-2 italic">
                                    "Note: Guests attending both locations are counted in each location."
                                </p>
                            </div>

                            {/* Dietary Restrictions */}
                            <div class=CARD>
                                <h3 class=SECTION_TITLE>
                                    "Dietary Restrictions"
                                </h3>
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <DietaryCard
                                        icon="🥗"
                                        label="Vegetarian"
                                        count=s.vegetarian_count
                                    />
                                    <DietaryCard
                                        icon="🌱"
                                        label="Vegan"
                                        count=s.vegan_count
                                    />
                                    <DietaryCard
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

#[component]
fn StatCard(
    icon: &'static str,
    title: &'static str,
    value: i32,
    color: &'static str,
) -> impl IntoView {
    let bg_color = match color {
        "blue" => "bg-blue-50",
        "green" => "bg-green-50",
        "yellow" => "bg-yellow-50",
        "purple" => "bg-purple-50",
        _ => "bg-gray-50",
    };

    let text_color = match color {
        "blue" => "text-blue-600",
        "green" => "text-green-600",
        "yellow" => "text-yellow-600",
        "purple" => "text-purple-600",
        _ => "text-gray-600",
    };

    view! {
        <div class={format!("rounded-lg shadow-md p-6 {}", bg_color)}>
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm font-medium text-gray-600 mb-1">{title}</p>
                    <p class={format!("text-3xl font-bold {}", text_color)}>{value}</p>
                </div>
                <div class="text-4xl">{icon}</div>
            </div>
        </div>
    }
}

#[component]
fn LocationCard(icon: &'static str, location: &'static str, count: i32) -> impl IntoView {
    view! {
        <div class=INFO_CARD>
            <div class="flex items-center space-x-3">
                <span class="text-2xl">{icon}</span>
                <div>
                    <p class="text-sm font-medium text-gray-600">{location}</p>
                    <p class="text-2xl font-bold text-gray-900">{count}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DietaryCard(icon: &'static str, label: &'static str, count: i32) -> impl IntoView {
    view! {
        <div class=INFO_CARD>
            <div class="flex items-center space-x-3">
                <span class="text-2xl">{icon}</span>
                <div>
                    <p class="text-sm font-medium text-gray-600">{label}</p>
                    <p class="text-2xl font-bold text-gray-900">{count}</p>
                </div>
            </div>
        </div>
    }
}
