use crate::contexts::use_guest_context;
use crate::i18n::{use_translations, Translations};
use crate::types::Location;
use leptos::*;

#[component]
pub fn EventsPage() -> impl IntoView {
    let guest_context = use_guest_context();
    let translations = use_translations();

    view! {
        <div class="max-w-4xl mx-auto">
            // Page Header
            <div class="text-center mb-6 animate-fade-in">
                <h1 class="text-5xl md:text-6xl font-serif font-light text-secondary-800 mb-6 tracking-wide">
                    {move || translations().t("events.title")}
                </h1>
                <div class="w-24 h-0.5 bg-primary-400 mx-auto mb-6"></div>
                <p class="text-lg md:text-xl text-secondary-600 font-light mb-8">
                    {move || {
                        let visible_count = [
                            (Location::Sardinia, "sardinia"),
                            (Location::Tunisia, "tunisia"),
                            (Location::Nice, "nice"),
                        ].iter().filter(|&&(ref loc, name)| guest_context.can_see_location(name) && !loc.is_past()).count();

                        if visible_count > 1 {
                            translations().t("events.subtitle_multiple")
                        } else {
                            translations().t("events.subtitle_single")
                        }
                    }}
                </p>
            </div>

            // Events Timeline
            <div class="space-y-12">
                {move || {
                    // Define all locations with their dates for sorting
                    let mut locations: Vec<(Location, &str, String)> = vec![
                        (Location::Sardinia, "events.sardinia", translations().t("events.sort_date_sardinia")),
                        (Location::Tunisia, "events.tunisia", translations().t("events.sort_date_tunisia")),
                        (Location::Nice, "events.nice", translations().t("events.sort_date_nice")),
                    ];

                    // Filter to only visible and non-past locations, then sort by date
                    locations.retain(|(loc, _, _)| guest_context.can_see_location(loc.as_str()) && !loc.is_past());
                    locations.sort_by(|a, b| a.2.cmp(&b.2));

                    // Render sorted locations
                    locations.into_iter().map(|(loc, title_key, _)| {
                        let flag = loc.flag_image();
                        let location = loc.as_str();
                        view! {
                            <LocationSection
                                location=location
                                title=move || translations().t(title_key)
                                flag=flag
                                translations=translations
                            />
                        }
                    }).collect_view()
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
        <div class="bg-white rounded-2xl shadow-sm border border-primary-100 overflow-hidden animate-fade-in">
            // Location Header
            <div class="bg-gradient-to-r from-primary-50 via-accent-50 to-primary-50 px-8 py-8 border-b border-primary-200">
                <div class="flex items-center gap-4 mb-2">
                    <img
                        src={flag}
                        alt="Flag"
                        class="w-16 h-12 object-cover rounded-md shadow-md border-2 border-white"
                    />
                    <div>
                        <h2 class="text-3xl md:text-4xl font-serif text-secondary-800 font-light">
                            {title}
                        </h2>
                        <p class="text-lg text-secondary-600 font-light mt-1">
                            {move || {
                                let date_key = format!("events.date_{}", location);
                                translations().t(&date_key)
                            }}
                        </p>
                    </div>
                </div>
            </div>

            // Event Details Grid
            <div class="p-8">
                <div class="grid md:grid-cols-2 gap-6">
                    <InfoCard
                        icon="🗓️"
                        title=move || translations().t("events.schedule")
                        content_key=format!("schedule_{}", location)
                        translations=translations
                    />

                    <InfoCard
                        icon="📍"
                        title=move || translations().t("events.venue")
                        content_key=format!("venue_{}", location)
                        translations=translations
                    />

                    <InfoCard
                        icon="🏨"
                        title=move || translations().t("events.accommodation")
                        content_key=format!("accommodation_{}", location)
                        translations=translations
                    />

                    <InfoCard
                        icon="✈️"
                        title=move || translations().t("events.travel")
                        content_key=format!("travel_{}", location)
                        translations=translations
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn InfoCard(
    icon: &'static str,
    title: impl Fn() -> String + 'static,
    content_key: String,
    translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    let is_venue = content_key.contains("venue");
    let is_schedule = content_key.contains("schedule");
    let content_key = store_value(content_key);

    let placeholder_content = move || {
        let key = format!("events.{}", content_key.get_value());
        translations().t(&key)
    };

    let venue_name = move || {
        let key = format!("events.{}_name", content_key.get_value());
        translations().t(&key)
    };

    let venue_link = move || {
        let key = format!("events.{}_link", content_key.get_value());
        translations().t(&key)
    };

    let schedule_date = move || {
        let location = content_key.get_value().replace("schedule_", "");
        let key = format!("events.date_{}", location);
        translations().t(&key)
    };

    view! {
        <div class="group bg-gradient-to-br from-primary-50/30 to-white rounded-xl p-6 border border-primary-100/50 hover:border-primary-300 hover:shadow-md transition-all duration-300">
            <div class="flex items-start gap-3 mb-4">
                <div class="text-4xl flex-shrink-0 transform group-hover:scale-110 transition-transform duration-300">
                    {icon}
                </div>
                <h3 class="text-xl font-serif text-secondary-800 pt-1 font-light">
                    {title}
                </h3>
            </div>

            {move || {
                if is_venue {
                    view! {
                        <div>
                            <p class="text-secondary-600 leading-relaxed mb-6 font-light">
                                {venue_name}
                            </p>
                            <div class="flex justify-end">
                                <a
                                    href=venue_link
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center gap-2 px-5 py-2.5 bg-secondary-600 hover:bg-secondary-700 text-white rounded-full shadow-sm hover:shadow-md transform hover:scale-105 transition-all duration-200 text-sm font-light"
                                >
                                    <span>"📍"</span>
                                    <span>{move || translations().t("events.view_on_maps")}</span>
                                    <span>"↗"</span>
                                </a>
                            </div>
                        </div>
                    }.into_view()
                } else if is_schedule {
                    view! {
                        <div>
                            <p class="text-secondary-800 font-medium mb-2">
                                {schedule_date}
                            </p>
                            <p class="text-secondary-600 leading-relaxed font-light">
                                {placeholder_content}
                            </p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <p class="text-secondary-600 leading-relaxed font-light" inner_html=placeholder_content>
                        </p>
                    }.into_view()
                }
            }}
        </div>
    }
}
