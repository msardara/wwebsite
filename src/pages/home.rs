use crate::contexts::GuestContext;
use crate::i18n::Translations;
use crate::styles::*;
use crate::types::Language;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");

    let translations = move || Translations::new(language.get());

    view! {
        <div class="max-w-3xl mx-auto">
            <div class="text-center mb-12 animate-fade-in">
                <h1 class="text-5xl md:text-6xl font-serif font-bold text-primary-600 mb-4">
                    {move || translations().t("home.title")}
                </h1>
                <p class="text-xl md:text-2xl text-gray-600 font-light">
                    {move || translations().t("home.subtitle")}
                </p>
            </div>

            <div class="bg-white rounded-lg shadow-lg p-8 md:p-12 mb-8 animate-fade-in">
                <div class="flex justify-center mb-8">
                    <div class="w-32 h-32 rounded-full bg-gradient-to-br from-primary-300 to-secondary-300 flex items-center justify-center text-6xl">
                        "💑"
                    </div>
                </div>

                <h2 class="text-3xl font-serif text-center mb-6 text-gray-800">
                    {move || translations().t("home.welcome")}
                </h2>

                <div class="prose prose-lg mx-auto text-gray-700">
                    <p class="text-center leading-relaxed">
                        {move || translations().t("home.intro_p1")}
                    </p>
                    <p class="text-center leading-relaxed mt-4">
                        {move || translations().t("home.intro_p2")}
                    </p>
                </div>
            </div>

            <div class="grid md:grid-cols-2 gap-6 mb-8">
                <Show when=move || guest_context.can_see_location("sardinia")>
                    <LocationCard
                        translations=translations
                        location_key="home.sardinia_title"
                        flag="🇮🇹"
                        description_key="home.sardinia_desc"
                        image_class="from-blue-300 to-green-300"
                    />
                </Show>
                {
                    let guest_ctx = guest_context;
                    view! {
                        <Show when=move || guest_ctx.can_see_location("tunisia")>
                            <LocationCard
                                translations=translations
                                location_key="home.tunisia_title"
                                flag="🇹🇳"
                                description_key="home.tunisia_desc"
                                image_class="from-yellow-300 to-red-300"
                            />
                        </Show>
                    }
                }
            </div>

            <div class="text-center animate-fade-in">
                <a
                    href="/rsvp"
                    class=BUTTON_PRIMARY_INLINE
                >
                    {move || translations().t("nav.rsvp")} " →"
                </a>
            </div>
        </div>
    }
}

#[component]
fn LocationCard(
    translations: impl Fn() -> Translations + 'static + Copy,
    location_key: &'static str,
    flag: &'static str,
    description_key: &'static str,
    image_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-xl transition-shadow duration-200 animate-fade-in">
            <div class=format!("w-full h-32 rounded-md bg-gradient-to-br {} flex items-center justify-center text-6xl mb-4", image_class)>
                {flag}
            </div>
            <h3 class="text-2xl font-serif font-bold text-gray-800 mb-2">
                {move || translations().t(location_key)}
            </h3>
            <p class="text-gray-600">
                {move || translations().t(description_key)}
            </p>
        </div>
    }
}
