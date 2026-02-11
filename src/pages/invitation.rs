use crate::contexts::{use_supabase_rpc, GuestContext};
use crate::i18n::Translations;
use crate::styles::*;
use crate::types::Language;
use crate::SupabaseError;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

use wasm_bindgen_futures::spawn_local;

#[component]
pub fn InvitationPage() -> impl IntoView {
    let language = use_context::<ReadSignal<Language>>().expect("Language context not found");
    let guest_context = use_context::<GuestContext>().expect("GuestContext not found");
    let translations = move || Translations::new(language.get());

    // Get query parameters to check for pre-filled code
    let query = use_query_map();
    let initial_code = query.with(|params| {
        params.get("code").cloned().unwrap_or_default().to_uppercase()
    });

    let (code, set_code) = create_signal(initial_code.clone());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let guest_ctx = guest_context; // Copy for closures

    // Auto-login if code is provided in URL
    create_effect(move |_| {
        let code_value = initial_code.clone();
        if !code_value.is_empty() {
            set_loading.set(true);
            set_error.set(None);

            let client = use_supabase_rpc();

            spawn_local(async move {
                match client.find_guest_by_code(&code_value).await {
                    Ok(Some(guest)) => {
                        set_loading.set(false);
                        guest_ctx.login(guest);
                        // Redirect to home page
                        let navigate = leptos_router::use_navigate();
                        navigate("/", Default::default());
                    }
                    Ok(None) => {
                        set_loading.set(false);
                        set_error.set(Some(translations().t("rsvp.error_code_invalid")));
                    }
                    Err(e) => {
                        set_loading.set(false);
                        let error_msg = match e {
                            SupabaseError::NetworkError(_) => translations().t("rsvp.error_network"),
                            SupabaseError::NotFound => translations().t("rsvp.not_found"),
                            _ => translations().t("rsvp.error_generic"),
                        };
                        set_error.set(Some(error_msg));
                    }
                }
            });
        }
    });

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let code_value = code.get();
        if code_value.is_empty() {
            set_error.set(Some(translations().t("rsvp.error_code_required")));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let client = use_supabase_rpc();

        spawn_local(async move {
            match client.find_guest_by_code(&code_value).await {
                Ok(Some(guest)) => {
                    set_loading.set(false);
                    guest_ctx.login(guest);
                    // Redirect to home page
                    let navigate = leptos_router::use_navigate();
                    navigate("/", Default::default());
                }
                Ok(None) => {
                    set_loading.set(false);
                    set_error.set(Some(translations().t("rsvp.error_code_invalid")));
                }
                Err(e) => {
                    set_loading.set(false);
                    let error_msg = match e {
                        SupabaseError::NetworkError(_) => translations().t("rsvp.error_network"),
                        SupabaseError::NotFound => translations().t("rsvp.not_found"),
                        _ => translations().t("rsvp.error_generic"),
                    };
                    set_error.set(Some(error_msg));
                }
            }
        });
    };

    view! {
        <div class="min-h-screen flex flex-col bg-background">
            <InvitationHeader language=language/>

            <main class="flex-grow flex items-center justify-center px-4 py-12">
                <div class="max-w-3xl w-full">
                    <div class="text-center mb-8 animate-fade-in">
                    <h1 class="text-4xl md:text-5xl font-serif font-bold text-primary-600 mb-2">
                        "💍"
                    </h1>
                    <h2 class="text-3xl md:text-4xl font-serif font-bold text-primary-600 mb-4">
                        {move || translations().t("home.title")}
                    </h2>
                    <p class="text-lg text-gray-600">
                        {move || translations().t("rsvp.subtitle")}
                    </p>
                </div>

                <div class="bg-white rounded-lg shadow-xl p-8 md:p-12 animate-fade-in">
                    <h3 class="text-2xl font-serif font-bold text-gray-800 mb-6 text-center">
                        {move || translations().t("rsvp.lookup")}
                    </h3>

                    <form on:submit=handle_submit class="space-y-6">
                        <div>
                            <label class="block text-sm font-semibold text-gray-700 mb-2">
                                {move || translations().t("rsvp.code")}
                            </label>
                            <input
                                type="text"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent transition-all uppercase text-center text-2xl tracking-widest"
                                placeholder=move || translations().t("rsvp.code_placeholder")
                                prop:value=code
                                on:input=move |ev| set_code.set(event_target_value(&ev).to_uppercase())
                                required
                                disabled=move || loading.get()
                                autofocus
                            />
                            <p class="text-sm text-gray-500 mt-2 text-center">
                                {move || translations().t("rsvp.code_help")}
                            </p>
                        </div>

                        <Show when=move || error.get().is_some()>
                            <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg animate-fade-in">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <button
                            type="submit"
                            class=BUTTON_PRIMARY
                            disabled=move || loading.get()
                        >
                            <Show
                                when=move || loading.get()
                                fallback=move || view! { <span>{move || translations().t("rsvp.find")}</span> }
                            >
                                <span class="flex items-center justify-center">
                                    <svg class="animate-spin h-5 w-5 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    {move || translations().t("common.loading")}
                                </span>
                            </Show>
                        </button>
                    </form>
                </div>

                    <p class="text-center text-sm text-gray-500 mt-6 flex justify-center gap-3">
                        <img src="/public/sardinia-flag.png" alt="Sardinia" class="w-8 h-6 object-cover rounded shadow-md border border-gray-300"/>
                        <img src="/public/tunisia-flag.png" alt="Tunisia" class="w-8 h-6 object-cover rounded shadow-md border border-gray-300"/>
                    </p>
                </div>
            </main>

            <InvitationFooter translations=translations/>
        </div>
    }
}

#[component]
fn InvitationHeader(language: ReadSignal<Language>) -> impl IntoView {
    let set_language = use_context::<WriteSignal<Language>>().expect("Language setter not found");

    let change_language = move |lang: Language| {
        set_language.set(lang);
        let _ = LocalStorage::set("language", lang.code());
    };

    view! {
        <header class="bg-white shadow-md sticky top-0 z-50">
            <nav class="container mx-auto max-w-5xl px-4 py-4">
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-serif font-bold text-primary-600">
                        "💍 Our Wedding"
                    </div>

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
                            on:click=move |_| change_language(Language::English)
                        >
                            "🇬🇧 EN"
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
                            on:click=move |_| change_language(Language::French)
                        >
                            "🇫🇷 FR"
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
                            on:click=move |_| change_language(Language::Italian)
                        >
                            "🇮🇹 IT"
                        </button>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn InvitationFooter(translations: impl Fn() -> Translations + 'static + Copy) -> impl IntoView {
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
