use crate::contexts::GuestContext;
use crate::i18n::Translations;
use crate::types::Language;
use leptos::*;

#[component]
pub fn EventsPage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");

    let translations = move || Translations::new(language.get());

    view! {
        <div class="max-w-3xl mx-auto">
            <div class="text-center mb-12 animate-fade-in">
                <h1 class="text-4xl md:text-5xl font-serif font-bold text-primary-600 mb-4">
                    {move || translations().t("events.title")}
                </h1>
                <p class="text-lg text-gray-600">
                    "Celebrate with us in two beautiful locations"
                </p>
            </div>

            <div class="space-y-12">
                {move || {
                    if guest_context.can_see_location("sardinia") {
                        view! {
                            <LocationSection
                                location="sardinia"
                                title=move || translations().t("events.sardinia")
                                flag="🇮🇹"
                                translations=translations
                            />
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}

                {move || {
                    if guest_context.can_see_location("tunisia") {
                        view! {
                            <LocationSection
                                location="tunisia"
                                title=move || translations().t("events.tunisia")
                                flag="🇹🇳"
                                translations=translations
                            />
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn LocationSection(
    location: &'static str,
    title: impl Fn() -> String + 'static + Copy,
    flag: &'static str,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-8 animate-fade-in">
            <div class="flex items-center mb-6">
                <span class="text-5xl mr-4">{flag}</span>
                <h2 class="text-3xl font-serif font-bold text-gray-800">
                    {title}
                </h2>
            </div>

            <div class="grid md:grid-cols-2 gap-8">
                <InfoCard
                    icon="📅"
                    title=move || translations().t("events.schedule")
                    content_key=format!("schedule_{}", location)
                />

                <InfoCard
                    icon="📍"
                    title=move || translations().t("events.venue")
                    content_key=format!("venue_{}", location)
                />

                <InfoCard
                    icon="🏨"
                    title=move || translations().t("events.accommodation")
                    content_key=format!("accommodation_{}", location)
                />

                <InfoCard
                    icon="✈️"
                    title=move || translations().t("events.travel")
                    content_key=format!("travel_{}", location)
                />
            </div>
        </div>
    }
}

#[component]
fn InfoCard(
    icon: &'static str,
    title: impl Fn() -> String + 'static,
    content_key: String,
) -> impl IntoView {
    // Placeholder content - will be loaded from Supabase in Phase 5
    let placeholder_content = match content_key.as_str() {
        "schedule_sardinia" => "Ceremony at 4:00 PM, Reception to follow at 6:00 PM",
        "venue_sardinia" => "Beautiful seaside venue in Costa Smeralda",
        "accommodation_sardinia" => "Recommended hotels nearby with special rates for our guests",
        "travel_sardinia" => "Olbia Airport (OLB) is the closest. We recommend renting a car.",
        "schedule_tunisia" => "Ceremony at 5:00 PM, Reception to follow at 7:00 PM",
        "venue_tunisia" => "Traditional venue in Tunis with stunning views",
        "accommodation_tunisia" => "Selection of hotels and guest houses in Tunis",
        "travel_tunisia" => "Tunis-Carthage International Airport (TUN) serves the area",
        _ => "Details coming soon!",
    };

    view! {
        <div class="bg-gray-50 rounded-lg p-6 hover:bg-gray-100 transition-colors duration-200">
            <div class="flex items-start mb-3">
                <span class="text-3xl mr-3">{icon}</span>
                <h3 class="text-xl font-serif font-semibold text-gray-800">
                    {title}
                </h3>
            </div>
            <p class="text-gray-600 leading-relaxed">
                {placeholder_content}
            </p>
        </div>
    }
}
