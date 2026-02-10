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
    let content_key = store_value(content_key);

    // Placeholder content - will be loaded from Supabase in Phase 5
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

    view! {
        <div class="bg-gray-50 rounded-lg p-6 hover:bg-gray-100 transition-colors duration-200">
            <div class="flex items-start mb-3">
                <span class="text-3xl mr-3">{icon}</span>
                <h3 class="text-xl font-serif font-semibold text-gray-800">
                    {title}
                </h3>
            </div>

            {move || {
                if is_venue {
                    view! {
                        <div>
                            <p class="text-gray-600 leading-relaxed mb-4">
                                {venue_name}
                            </p>
                            <div class="flex justify-end mt-4">
                                <a
                                    href=venue_link
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="inline-flex items-center gap-2 px-4 py-2 bg-primary-500 hover:bg-primary-600 text-white rounded-lg shadow-md hover:shadow-lg transform hover:scale-[1.02] transition-all duration-200 font-medium text-sm"
                                >
                                    "📍 " {move || translations().t("events.view_on_maps")} " ↗"
                                </a>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <p class="text-gray-600 leading-relaxed">
                            {placeholder_content}
                        </p>
                    }.into_view()
                }
            }}
        </div>
    }
}
