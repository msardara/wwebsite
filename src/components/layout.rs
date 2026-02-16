use crate::components::common::LanguageSelector;
use crate::contexts::{use_guest_context, use_language};
use crate::i18n::use_translations;
use crate::types::Language;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout() -> impl IntoView {
    let guest_context = use_guest_context();
    let t = use_translations();

    // Show loading while checking authentication
    let (auth_checked, set_auth_checked) = create_signal(false);

    // Check authentication once on mount
    create_effect(move |_| {
        if !guest_context.is_authenticated() {
            let navigate = use_navigate();
            navigate("/invitation", Default::default());
        } else {
            set_auth_checked.set(true);
        }
    });

    let (language, change_language) = use_language();

    let (mobile_menu_open, set_mobile_menu_open) = create_signal(false);

    view! {
        <Show
            when=move || auth_checked.get()
            fallback=move || view! {
                <div class="min-h-screen flex items-center justify-center">
                    <div class="text-center">
                        <div class="text-4xl mb-4">"💍"</div>
                        <p class="text-gray-600">{move || t().t("common.loading")}</p>
                    </div>
                </div>
            }
        >
            <div class="min-h-screen flex flex-col bg-primary-50/50">
                <Header
                    language=language
                    on_language_change=change_language
                    mobile_menu_open=mobile_menu_open
                    set_mobile_menu_open=set_mobile_menu_open
                />

                <main class="flex-grow container mx-auto max-w-6xl px-4 py-8">
                    <Outlet/>
                </main>

                <Footer/>
            </div>
        </Show>
    }
}

#[component]
fn Header(
    language: ReadSignal<Language>,
    on_language_change: impl Fn(Language) + 'static + Copy,
    mobile_menu_open: ReadSignal<bool>,
    set_mobile_menu_open: WriteSignal<bool>,
) -> impl IntoView {
    let guest_context = use_guest_context();
    let t = use_translations();
    let guest_ctx = guest_context; // Copy for closures
    let location = use_location();

    let is_active = move |path: &str| location.pathname.get() == path;

    let nav_link_class = move |path: &str| {
        let base = "px-5 py-2.5 rounded-full transition-all duration-300 font-light tracking-wide text-sm ";
        if is_active(path) {
            format!("{}bg-secondary-700 text-primary-50 shadow-md", base)
        } else {
            format!(
                "{}text-secondary-800 hover:bg-secondary-200/50 hover:text-secondary-900",
                base
            )
        }
    };

    view! {
        <header class="bg-white shadow-md border-b border-secondary-300 sticky top-0 z-50">
            <nav class="container mx-auto max-w-6xl px-6 py-6">
                <div class="flex items-center justify-between">
                    // Logo/Brand
                    <A href="/" class="text-2xl md:text-3xl font-serif font-light text-secondary-800 hover:text-secondary-700 transition-colors tracking-wider">
                        "💍 Our Wedding"
                    </A>

                    // Desktop Navigation
                    <div class="hidden md:flex items-center space-x-1">
                        <A href="/" class=move || nav_link_class("/")>
                            {move || t().t("nav.home")}
                        </A>
                        <A href="/events" class=move || nav_link_class("/events")>
                            {move || t().t("nav.events")}
                        </A>
                        <A href="/rsvp" class=move || nav_link_class("/rsvp")>
                            {move || t().t("nav.rsvp")}
                        </A>

                        <div class="mx-6 border-l border-r border-secondary-300/40 px-6">
                            <LanguageSelector language=language on_change=on_language_change/>
                        </div>

                        // Logout button
                        <div class="ml-6 border-l border-secondary-300/40 pl-6 flex items-center">
                            <Show when=move || guest_ctx.guest.get().is_some()>
                                <button
                                    class="text-xs px-4 py-2 bg-secondary-200/50 hover:bg-secondary-300/60 rounded-full transition-all duration-200 font-light tracking-wide text-secondary-800"
                                    on:click=move |_| {
                                        guest_ctx.logout();
                                        let navigate = use_navigate();
                                        navigate("/invitation", Default::default());
                                    }
                                >
                                    {move || t().t("admin.logout")}
                                </button>
                            </Show>
                        </div>
                    </div>

                    // Mobile Menu Button
                    <button
                        class="md:hidden p-2 rounded-lg hover:bg-secondary-200/50 transition-colors text-secondary-800"
                        on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h16M4 18h16"
                            />
                        </svg>
                    </button>
                </div>

                // Mobile Navigation
                <div class=move || {
                    if mobile_menu_open.get() {
                        "md:hidden mt-6 pb-4 space-y-2 animate-slide-in bg-white/60 rounded-lg p-4 backdrop-blur-sm"
                    } else {
                        "hidden"
                    }
                }>
                    <A
                        href="/"
                        class="block px-5 py-3 rounded-lg text-secondary-800 hover:bg-secondary-200/50 font-light tracking-wide transition-all"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || t().t("nav.home")}
                    </A>
                    <A
                        href="/events"
                        class="block px-5 py-3 rounded-lg text-secondary-800 hover:bg-secondary-200/50 font-light tracking-wide transition-all"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || t().t("nav.events")}
                    </A>
                    <A
                        href="/rsvp"
                        class="block px-5 py-3 rounded-lg text-secondary-800 hover:bg-secondary-200/50 font-light tracking-wide transition-all"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || t().t("nav.rsvp")}
                    </A>

                    <div class="pt-3 mt-3 border-t border-secondary-300/40">
                        <LanguageSelector language=language on_change=on_language_change/>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let t = use_translations();

    view! {
        <footer class="bg-primary-50 border-t border-secondary-200/40">
            <div class="container mx-auto max-w-6xl px-6 py-10">
                <div class="text-center text-secondary-700">
                    <p class="text-xs md:text-sm font-light tracking-wide">
                        {move || t().t("footer.copyright")}
                    </p>
                </div>
            </div>
        </footer>
    }
}
