use crate::contexts::GuestContext;
use crate::i18n::Translations;
use crate::types::Language;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");

    let translations = move || Translations::new(language.get());

    view! {
        <div class="w-full -mt-8">
            // Hero Banner Image - Full Width
            <div class="relative w-full h-[70vh] md:h-[75vh] overflow-hidden animate-fade-in">
                <img
                    src="/public/decoration-1.png"
                    alt="Hero"
                    class="w-full h-full object-cover object-center"
                />
                <div class="absolute inset-0 bg-gradient-to-b from-secondary-900/20 via-transparent to-secondary-900/50"></div>

                // Blue sky background at top
                <div class="absolute top-0 left-0 right-0 h-32 md:h-40 bg-gradient-to-b from-sky-400 to-sky-300/0" style="background: linear-gradient(to bottom, #87CEEB 0%, #87CEEB 30%, rgba(135, 206, 235, 0) 100%);"></div>

                <div class="absolute top-0 left-0 right-0 flex justify-center pt-4 md:pt-6">
                    <div class="text-center text-white px-4 pb-4 md:px-6 md:pb-6 max-w-3xl w-full md:w-auto md:mx-4">
                        <h1 class="text-3xl sm:text-4xl md:text-6xl lg:text-7xl font-normal mb-0 md:mb-1 drop-shadow-2xl leading-tight" style="font-family: 'Great Vibes', 'Dancing Script', 'Brush Script MT', cursive; color: rgb(243, 246, 250); font-kerning: none; text-decoration: none;">
                            "Mauro & Mouna"
                        </h1>
                        <p class="text-xs sm:text-sm md:text-xl lg:text-2xl font-normal tracking-wider md:tracking-widest drop-shadow-lg opacity-95 uppercase" style="font-family: 'Cinzel', 'Playfair Display', Georgia, serif; color: rgb(243, 246, 250); font-kerning: none; text-decoration: none; letter-spacing: 0.15em;">
                            {move || translations().t("home.subtitle")}
                        </p>
                    </div>
                </div>
            </div>

            // Couple Photo Section with Side Text - Dark Background
            <div class="bg-secondary-900 py-20 md:py-28 px-6 mt-16 md:mt-24">
                <div class="max-w-6xl mx-auto">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12 items-center">
                        // Left Text
                        <div class="text-center md:text-right text-primary-50 order-2 md:order-1">
                            <p class="text-lg md:text-xl font-serif font-light tracking-wider uppercase mb-2">
                                "Our Love"
                            </p>
                            <p class="text-base md:text-lg font-serif font-light tracking-wider uppercase text-primary-100">
                                "For Gardens"
                            </p>
                        </div>

                        // Center - Couple Photo
                        <div class="flex justify-center order-1 md:order-2">
                            <div class="w-64 md:w-80 lg:w-96 overflow-hidden shadow-2xl">
                                <img
                                    src="/public/decoration-1.png"
                                    alt="Couple"
                                    class="w-full h-auto object-cover"
                                />
                            </div>
                        </div>

                        // Right Text
                        <div class="text-center md:text-left text-primary-50 order-3">
                            <p class="text-lg md:text-xl font-serif font-light tracking-wider uppercase mb-2">
                                "(And Each"
                            </p>
                            <p class="text-base md:text-lg font-serif font-light tracking-wider uppercase text-primary-100">
                                "Other)"
                            </p>
                        </div>
                    </div>

                    // Description Text Below
                    <div class="mt-16 text-center max-w-3xl mx-auto">
                        <p class="text-primary-100/90 text-sm md:text-base lg:text-lg font-light leading-relaxed tracking-wide">
                            "Write a paragraph that tells your story as a couple. You can include details like how you met, your journey together, and what makes your relationship unique. This is your chance to share your personality and connect with your guests."
                        </p>
                    </div>
                </div>
            </div>

            // Mirror Photo Section
            <div class="bg-primary-50 py-20 md:py-28 px-6 mt-16 md:mt-24">
                <div class="max-w-6xl mx-auto flex justify-center">
                    <div class="relative w-full max-w-4xl">
                        <img
                            src="/public/decoration-2.png"
                            alt="Mirror reflection"
                            class="w-full h-auto object-contain"
                            style="user-select: none; -webkit-tap-highlight-color: transparent; box-shadow: 0px 0px 0px 0.5px rgba(64,87,109,0.06), 0px 2px 4px 0px rgba(24,44,89,0.14), 0px 6px 12px 0px rgba(24,44,89,0.07);"
                        />
                    </div>
                </div>
            </div>

            // Locations Section with Full Width Image
            <div class="relative w-full h-[45vh] md:h-[55vh] overflow-hidden">
                <Show when=move || guest_context.can_see_location("sardinia")>
                    <img
                        src="/public/cala-luna.jpg"
                        alt="Location"
                        class="w-full h-full object-cover object-center"
                    />
                </Show>
                <div class="absolute inset-0 bg-gradient-to-b from-secondary-900/30 to-secondary-900/50"></div>
            </div>



            // RSVP Call to Action - Dark Section
            <div class="bg-secondary-800 py-20 md:py-28 px-6 mt-16 md:mt-24">
                <div class="max-w-3xl mx-auto text-center">
                    <h2 class="text-2xl md:text-3xl lg:text-4xl font-serif text-primary-50 mb-4 font-light tracking-wide">
                        "Your presence is truly the best"
                    </h2>
                    <h3 class="text-xl md:text-2xl lg:text-3xl font-serif text-primary-100 mb-10 font-light tracking-wide">
                        "gift we could ask for"
                    </h3>
                    <p class="text-primary-100/90 text-sm md:text-base lg:text-lg mb-12 max-w-2xl mx-auto font-light leading-relaxed">
                        "But if you feel inclined to give a little something extra, a contribution toward our future together would be warmly appreciated. We're saving up for a special trip and every little bit helps us on our way!"
                    </p>
                    <a
                        href="/rsvp"
                        class="inline-block bg-primary-400 hover:bg-primary-500 text-white font-light tracking-wide py-3.5 px-8 md:px-12 rounded-full shadow-md hover:shadow-xl transform hover:scale-[1.03] transition-all duration-300 text-base md:text-lg"
                    >
                        {move || translations().t("nav.rsvp")} " →"
                    </a>
                </div>
            </div>


        </div>
    }
}
