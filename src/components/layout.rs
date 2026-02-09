use crate::contexts::GuestContext;
use crate::types::Language;
use leptos::*;
use leptos_router::*;

use crate::i18n::Translations;
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn Layout() -> impl IntoView {
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");

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

    // Get language context from app level
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let set_language =
        use_context::<WriteSignal<Language>>().expect("Language setter context not found");

    let translations = move || Translations::new(language.get());

    // Language change handler with localStorage persistence
    let change_language = move |lang: Language| {
        set_language.set(lang);
        let _ = LocalStorage::set("language", lang.code());
    };

    let (mobile_menu_open, set_mobile_menu_open) = create_signal(false);

    view! {
        <Show
            when=move || auth_checked.get()
            fallback=move || view! {
                <div class="min-h-screen flex items-center justify-center">
                    <div class="text-center">
                        <div class="text-4xl mb-4">"💍"</div>
                        <p class="text-gray-600">{move || translations().t("common.loading")}</p>
                    </div>
                </div>
            }
        >
            <div class="min-h-screen flex flex-col bg-background">
                <Header
                    language=language
                    on_language_change=change_language
                    translations=translations
                    mobile_menu_open=mobile_menu_open
                    set_mobile_menu_open=set_mobile_menu_open
                />

                <main class="flex-grow container mx-auto max-w-5xl px-4 py-8">
                    <Outlet/>
                </main>

                <Footer translations=translations/>
            </div>
        </Show>
    }
}

#[component]
fn Header(
    language: ReadSignal<Language>,
    on_language_change: impl Fn(Language) + 'static + Copy,
    translations: impl Fn() -> Translations + 'static + Copy,
    mobile_menu_open: ReadSignal<bool>,
    set_mobile_menu_open: WriteSignal<bool>,
) -> impl IntoView {
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");
    let guest_ctx = guest_context; // Copy for closures
    let location = use_location();

    let is_active = move |path: &str| location.pathname.get() == path;

    let nav_link_class = move |path: &str| {
        let base = "px-4 py-2 rounded-md transition-colors duration-200 ";
        if is_active(path) {
            format!("{}bg-primary-400 text-white", base)
        } else {
            format!("{}text-gray-700 hover:bg-primary-100", base)
        }
    };

    view! {
        <header class="bg-white shadow-md sticky top-0 z-50">
            <nav class="container mx-auto max-w-5xl px-4 py-4">
                <div class="flex items-center justify-between">
                    // Logo/Brand
                    <A href="/" class="text-2xl font-serif font-bold text-primary-600 hover:text-primary-700 transition-colors">
                        "💍 Our Wedding"
                    </A>

                    // Desktop Navigation
                    <div class="hidden md:flex items-center space-x-2">
                        <A href="/" class=move || nav_link_class("/")>
                            {move || translations().t("nav.home")}
                        </A>
                        <A href="/events" class=move || nav_link_class("/events")>
                            {move || translations().t("nav.events")}
                        </A>
                        <A href="/gallery" class=move || nav_link_class("/gallery")>
                            {move || translations().t("nav.gallery")}
                        </A>
                        <A href="/rsvp" class=move || nav_link_class("/rsvp")>
                            {move || translations().t("nav.rsvp")}
                        </A>

                        <div class="ml-4 border-l border-gray-300 pl-4">
                            <LanguageSelector language=language on_change=on_language_change/>
                        </div>

                        // Guest name and logout
                        <div class="ml-4 border-l border-gray-300 pl-4 flex items-center space-x-2">
                            <Show when=move || guest_ctx.guest.get().is_some()>
                                <span class="text-sm text-gray-600">
                                    {move || guest_ctx.guest.get().map(|g| g.name).unwrap_or_default()}
                                </span>
                                <button
                                    class="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 rounded transition-colors"
                                    on:click=move |_| {
                                        guest_ctx.logout();
                                        let navigate = use_navigate();
                                        navigate("/invitation", Default::default());
                                    }
                                >
                                    {move || translations().t("admin.logout")}
                                </button>
                            </Show>
                        </div>
                    </div>

                    // Mobile Menu Button
                    <button
                        class="md:hidden p-2 rounded-md hover:bg-gray-100 transition-colors"
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
                        "md:hidden mt-4 pb-4 space-y-2 animate-slide-in"
                    } else {
                        "hidden"
                    }
                }>
                    <A
                        href="/"
                        class="block px-4 py-2 rounded-md text-gray-700 hover:bg-primary-100"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || translations().t("nav.home")}
                    </A>
                    <A
                        href="/events"
                        class="block px-4 py-2 rounded-md text-gray-700 hover:bg-primary-100"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || translations().t("nav.events")}
                    </A>
                    <A
                        href="/gallery"
                        class="block px-4 py-2 rounded-md text-gray-700 hover:bg-primary-100"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || translations().t("nav.gallery")}
                    </A>
                    <A
                        href="/rsvp"
                        class="block px-4 py-2 rounded-md text-gray-700 hover:bg-primary-100"
                        on:click=move |_| set_mobile_menu_open.set(false)
                    >
                        {move || translations().t("nav.rsvp")}
                    </A>

                    <div class="pt-2 mt-2 border-t border-gray-200">
                        <LanguageSelector language=language on_change=on_language_change/>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn LanguageSelector(
    language: ReadSignal<Language>,
    on_change: impl Fn(Language) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="flex items-center space-x-2">
            <button
                class=move || {
                    let base = "px-3 py-1 rounded-md text-sm transition-all duration-200 ";
                    if language.get() == Language::English {
                        format!("{}bg-primary-400 text-white scale-110", base)
                    } else {
                        format!("{}bg-gray-100 hover:bg-gray-200", base)
                    }
                }
                on:click=move |_| on_change(Language::English)
                title="English"
            >
                "🇬🇧"
            </button>
            <button
                class=move || {
                    let base = "px-3 py-1 rounded-md text-sm transition-all duration-200 ";
                    if language.get() == Language::French {
                        format!("{}bg-primary-400 text-white scale-110", base)
                    } else {
                        format!("{}bg-gray-100 hover:bg-gray-200", base)
                    }
                }
                on:click=move |_| on_change(Language::French)
                title="Français"
            >
                "🇫🇷"
            </button>
            <button
                class=move || {
                    let base = "px-3 py-1 rounded-md text-sm transition-all duration-200 ";
                    if language.get() == Language::Italian {
                        format!("{}bg-primary-400 text-white scale-110", base)
                    } else {
                        format!("{}bg-gray-100 hover:bg-gray-200", base)
                    }
                }
                on:click=move |_| on_change(Language::Italian)
                title="Italiano"
            >
                "🇮🇹"
            </button>
        </div>
    }
}

#[component]
fn Footer(
    #[allow(unused_variables)] translations: impl Fn() -> Translations + 'static + Copy,
) -> impl IntoView {
    view! {
        <footer class="bg-white border-t border-gray-200 mt-12">
            <div class="container mx-auto max-w-5xl px-4 py-8">
                <div class="text-center text-gray-600">
                    <p class="text-sm">
                        {move || translations().t("footer.copyright")}
                    </p>
                </div>
            </div>
        </footer>
    }
}
