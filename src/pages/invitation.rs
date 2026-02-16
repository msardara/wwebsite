use crate::components::common::{LanguageSelector, LoadingButton};
use crate::contexts::{use_guest_context, use_language, use_supabase_rpc, GuestContext};
use crate::i18n::{use_translations, Translations};
use crate::styles::*;
use crate::supabase::SupabaseRpcClient;
use crate::SupabaseError;
use leptos::*;
use leptos_router::*;

use wasm_bindgen_futures::spawn_local;

/// Shared lookup logic used by both auto-login (URL code) and the manual form submit.
///
/// On success the guest is logged in, signals are updated, and the caller is
/// redirected to `/`. On failure the appropriate translated error is set.
async fn lookup_guest_by_code(
    code: &str,
    client: &SupabaseRpcClient,
    guest_ctx: GuestContext,
    t: impl Fn() -> Translations,
    set_loading: WriteSignal<bool>,
    set_error: WriteSignal<Option<String>>,
) {
    match client.find_guest_by_code(code).await {
        Ok(Some(guest)) => {
            set_loading.set(false);
            guest_ctx.login(guest);
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        }
        Ok(None) => {
            set_loading.set(false);
            set_error.set(Some(t().t("rsvp.error_code_invalid")));
        }
        Err(e) => {
            set_loading.set(false);
            let error_msg = match e {
                SupabaseError::NetworkError(_) => t().t("rsvp.error_network"),
                SupabaseError::NotFound => t().t("rsvp.not_found"),
                _ => t().t("rsvp.error_generic"),
            };
            set_error.set(Some(error_msg));
        }
    }
}

#[component]
pub fn InvitationPage() -> impl IntoView {
    let guest_context = use_guest_context();
    let t = use_translations();

    // Get query parameters to check for pre-filled code
    let query = use_query_map();
    let initial_code = query.with(|params| params.get("code").cloned().unwrap_or_default());

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
                lookup_guest_by_code(&code_value, &client, guest_ctx, t, set_loading, set_error)
                    .await;
            });
        }
    });

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let code_value = code.get();
        if code_value.is_empty() {
            set_error.set(Some(t().t("rsvp.error_code_required")));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let client = use_supabase_rpc();

        spawn_local(async move {
            lookup_guest_by_code(&code_value, &client, guest_ctx, t, set_loading, set_error).await;
        });
    };

    view! {
        <div class="min-h-screen flex flex-col bg-background">
            <InvitationHeader/>

            <main class="flex-grow flex items-center justify-center px-4 py-12">
                <div class="max-w-3xl w-full">
                    <div class="text-center mb-8 animate-fade-in">
                    <h1 class="text-4xl md:text-5xl font-serif font-bold text-primary-600 mb-2">
                        "💍"
                    </h1>
                    <h2 class="text-3xl md:text-4xl font-serif font-bold text-primary-600 mb-4">
                        {move || t().t("home.title")}
                    </h2>
                    <p class="text-lg text-gray-600">
                        {move || t().t("rsvp.subtitle")}
                    </p>
                </div>

                <div class="bg-white rounded-lg shadow-xl p-8 md:p-12 animate-fade-in">
                    <h3 class="text-2xl font-serif font-bold text-gray-800 mb-6 text-center">
                        {move || t().t("rsvp.lookup")}
                    </h3>

                    <form on:submit=handle_submit class="space-y-6">
                        <div>
                            <label class="block text-sm font-semibold text-gray-700 mb-2">
                                {move || t().t("rsvp.code")}
                            </label>
                            <input
                                type="text"
                                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent transition-all text-center text-2xl tracking-widest"
                                placeholder=move || t().t("rsvp.code_placeholder")
                                prop:value=code
                                on:input=move |ev| set_code.set(event_target_value(&ev))
                                required
                                disabled=move || loading.get()
                                autofocus
                            />
                            <p class="text-sm text-gray-500 mt-2 text-center">
                                {move || t().t("rsvp.code_help")}
                            </p>
                        </div>

                        <Show when=move || error.get().is_some()>
                            <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg animate-fade-in">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <LoadingButton
                            loading=move || loading.get()
                            label=move || t().t("rsvp.find")
                            class=BUTTON_PRIMARY
                        />
                    </form>
                </div>

                    <p class="text-center text-sm text-gray-500 mt-6 flex justify-center gap-3">
                        <img src="/public/sardinia-flag.png" alt="Sardinia" class="w-8 h-6 object-cover rounded shadow-md border border-gray-300"/>
                        <img src="/public/tunisia-flag.png" alt="Tunisia" class="w-8 h-6 object-cover rounded shadow-md border border-gray-300"/>
                    </p>
                </div>
            </main>

            <InvitationFooter/>
        </div>
    }
}

#[component]
fn InvitationHeader() -> impl IntoView {
    let (language, change_language) = use_language();

    view! {
        <header class="bg-white shadow-md sticky top-0 z-50">
            <nav class="container mx-auto max-w-5xl px-4 py-4">
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-serif font-bold text-primary-600">
                        "💍 Our Wedding"
                    </div>

                    <LanguageSelector language=language on_change=change_language/>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn InvitationFooter() -> impl IntoView {
    let t = use_translations();

    view! {
        <footer class="bg-white border-t border-gray-200 mt-12">
            <div class="container mx-auto max-w-5xl px-4 py-8">
                <div class="text-center text-gray-600">
                    <p class="text-sm">
                        {move || t().t("footer.copyright")}
                    </p>
                </div>
            </div>
        </footer>
    }
}
